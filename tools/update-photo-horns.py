#!/usr/bin/env python3
"""Rebuild the two photo-estimated horn recipes from recorded pixel calibration.

Printed annotations constrain the major dimensions. Inferred contour and hole
positions stay in estimates, not in the formal source-dimension/hole schema.
"""
import json
import math
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SOURCE = 'horn-user-aliexpress-photo'


def line(a, b):
    return {'kind': 'line', 'start_mm': a, 'end_mm': b}


def arc(center, a, b):
    return {'kind': 'arc', 'center_mm': center, 'start_mm': a, 'end_mm': b, 'ccw': True}


def cylinder(x, y, z, diameter, height):
    return {'kind': 'cylinder', 'base_mm': [x, y, z], 'axis': [0, 0, 1], 'diameter_mm': diameter, 'height_mm': height}


def estimate(value, unit, method, locator, note, pixels=()):
    return {'value': value, 'unit': unit,
            'evidence': {'source': SOURCE, 'locator': locator, 'method': method, 'note': note},
            'source_pixel_points': list(pixels), 'uncertainty_mm': None}


def rebuild():
    path = ROOT / 'catalog/horns-user-reference.json'
    pack = json.loads(path.read_text())
    pack['release'] = '2026-11-19.10'
    for horn in pack['horns']:
        double = horn['shape'] == 'double-arm'
        length, plate, height, hub = (31.2, 2.36, 4.72, 6.74) if double else (21.3, 2.08, 4.82, 6.76)
        tip_radius = 1.90 if double else 1.96
        reach = length / 2 if double else length - hub / 2
        tip_center = reach - tip_radius
        angle = math.radians(3.24) if double else math.asin((hub / 2 - tip_radius) / tip_center)
        dx, dy = tip_radius * math.sin(angle), tip_radius * math.cos(angle)
        right_upper, right_lower = [tip_center + dx, dy], [tip_center + dx, -dy]
        if double:
            intercept = tip_center * math.tan(angle) + tip_radius / math.cos(angle)
            left_upper, left_lower = [-tip_center - dx, dy], [-tip_center - dx, -dy]
            segments = [line([0, intercept], left_upper), arc([-tip_center, 0], left_upper, left_lower),
                        line(left_lower, [0, -intercept]), line([0, -intercept], right_lower),
                        arc([tip_center, 0], right_lower, right_upper), line(right_upper, [0, intercept])]
            radii = [4.41, 6.17, 8.04, 9.95, 11.82, 13.73]
            pixel_rows = [(588.0, 788.0), (551.7, 832.0), (510.0, 875.0), (468.4, 920.0), (426.5, 963.1), (385.0, 1007.8)]
            cavity = 4.65
        else:
            root_radius = hub / 2
            root_upper = [root_radius * math.sin(angle), root_radius * math.cos(angle)]
            root_lower = [root_upper[0], -root_upper[1]]
            segments = [arc([0, 0], root_upper, root_lower), line(root_lower, right_lower),
                        arc([tip_center, 0], right_lower, right_upper), line(right_upper, root_upper)]
            intercept = root_radius / math.cos(angle)
            radii = [4.65, 6.56, 8.44, 10.32, 12.26, 14.20, 16.12]
            pixel_rows = [1615.9, 1664.3, 1712.2, 1759.9, 1809.2, 1858.6, 1907.2]
            cavity = 4.54
        horn['model'] = 'SG90-style ' + ('double arm' if double else 'single arm') + ' / photo estimate'
        horn['data_revision'] = 2
        horn['status'] = 'photo_estimate'
        # Additional printed values remain annotations with their original ambiguity.
        for key, value, locator in [
                ('reported_link_opening_span', 1.45, 'bracket across a small linkage opening'),
                ('reported_center_opening_span', 2.36, 'bracket at the central clear opening'),
                ('reported_inner_hub_span', cavity, 'bracket inside the toothed hub opening'),
                ('reported_local_hole_spacing', 2.43, 'bracket between two tip-side linkage centers')]:
            horn['dimensions'][key] = {'nominal_mm': value, 'tolerance_mm': None, 'evidence': {
                'source': SOURCE, 'locator': ('double-arm' if double else 'single-arm') + ' photograph: ' + locator,
                'method': 'photo_annotation', 'note': 'Printed manual-measurement annotation; support convention and measurement error are unspecified. Its model interpretation is recorded separately in estimates.'}}
        horn['estimates'] = {
            'tip_radius': estimate(tip_radius, 'mm', 'photo_trace', 'Rounded end silhouette relative to hub span',
                'Fit circular tips to the photographed rounded silhouette after lateral calibration by reported hub span; regularized to1.90mm(double) or1.96mm(single). Perspective and physical error are not bounded.'),
            'arm_reach': estimate(reach, 'mm', 'photo_inference', 'Printed overall length and chosen shaft-centered reconstruction',
                'Double assumes centered equal reaches L/2. Single assumes the rear plan extremity is the reported circular hub radius, so reach=L-D/2.'),
            'tip_center': estimate(tip_center, 'mm', 'photo_inference', 'Analytic tangent tip construction', 'Tip-center X = inferred reach minus photo-calibrated tip radius.'),
            'half_taper': estimate(math.degrees(angle), 'deg', 'photo_trace', 'Arm width samples along original photograph',
                'Double half-angle3.24deg fitted to observed widths. Single fitted4.89deg is regularized to the common external tangent of hubR3.38 and tipR1.96 at X15.96, about5.10deg. This preserves smooth joins and reported major spans.'),
            'flank_intercept': estimate(intercept, 'mm', 'photo_inference', 'Analytic tangent line',
                'Double b=C*tan(alpha)+r/cos(alpha); single b=Rhub/cos(alpha). The arm is tapered, not a constant-width capsule.'),
            'link_hole_diameter': estimate(1.45, 'mm', 'photo_inference', 'Printed1.45 opening span plus visibly repeated openings',
                'Interpret the printed opening span as a circular linkage-hole diameter and apply it to the visible repeated holes. Diameter convention and physical fit remain unverified.'),
            'link_hole_count': estimate(12 if double else 7, 'count', 'photo_trace', 'Plan and side views of this photograph',
                'Double has six through openings per arm; single has seven. Paired off-axis molded dimples are not through holes.'),
            'center_clearance': estimate(2.36, 'mm', 'photo_inference', 'Printed central opening span',
                'Interpret as a coaxial circular retaining-clearance opening. No screw thread or fit is inferred.'),
            'cup_diameter': estimate(cavity, 'mm', 'photo_inference', 'Printed inner hub opening span',
                'Represent the visible toothed opening by a smooth visual circular envelope at the annotated span. Teeth, spline major/minor definitions and mating fit are not reconstructed.'),
            'cup_depth': estimate(height - plate, 'mm', 'photo_inference', 'Side-view plate and total-height annotations',
                'Cup opens on the mating side and stops at the underside of the arm plate: depth=reported total height minus reported plate span. Its true floor depth is not measured.'),
        }
        for i, radius in enumerate(radii, 1):
            if double:
                upper, lower = pixel_rows[i - 1]
                pixels = [[398.0, 689.5], [398.0, upper], [398.0, lower]]
                note = 'Mirror-average of upper/lower aperture radii after separate affine scales: top15.6/(689.5-344), bottom15.6/(1052-689.5)mm/pixel. Regularized to two decimal places, not a manufacturing tolerance.'
            else:
                pixels = [[413.0, 1497.7], [413.0, pixel_rows[i - 1]], [413.0, 1953.0]]
                note = 'Map aperture centroid using x=(pixel_y-1497.7)*17.92/(1953-1497.7). Perspective remains uncorrected; these are inferred centers, not source-dimensioned datums.'
            horn['estimates'][f'link_radius_{i}'] = estimate(radius, 'mm', 'photo_trace', 'Original1600x2706 image centerline aperture centroids', note, pixels)
        modifiers = [{'operation': 'join', 'geometry': cylinder(0, 0, -height, hub, height)}]
        for sign in ((-1, 1) if double else (1,)):
            for radius in radii:
                modifiers.append({'operation': 'cut', 'geometry': cylinder(sign * radius, 0, -plate, 1.45, plate)})
        modifiers += [{'operation': 'cut', 'geometry': cylinder(0, 0, -height, 2.36, height)},
                      {'operation': 'cut', 'geometry': cylinder(0, 0, -height, cavity, height - plate)}]
        horn['parts'] = [{'id': 'horn', 'label': 'tapered photo reconstruction / inferred holes and cup',
            'basis': 'photo_estimate', 'note': 'Printed major dimensions constrain a calibrated tapered line/arc contour. Hole centers and cup interpretation are source-linked estimates, not arbitrary constant-width strips. The hollow boss faces the actuator; its floor is inferred at the plate underside. Spline teeth and cosmetic molding dimples are omitted.',
            'color': '#c99545', 'geometry': {'kind': 'profile', 'z_mm': -plate, 'height_mm': plate, 'segments': segments},
            'modifiers': modifiers}]
        horn['holes'] = []
        horn['assumptions'] = [
            'Photo-based reconstruction, not a manufacturer-controlled drawing or a verified replacement part. Printed decimal places do not establish physical accuracy.',
            'Major length, plate span, axial span and hub span use the photo annotations; tapered contour and hole centers are calibrated inferences recorded per feature.',
            'The printed2.43 local spacing is retained as an annotation, not repeated as a global pitch: it would conflict with the visible hole counts and available arm reach.',
            'The hollow boss opens toward negative localZ, the actuator side. The smooth cup omits teeth; depth and retaining clearance interpretation remain inferred.',
            'Neutral phase, installed shaft height, specimen identity and compatibility with a particular SG90/SG92R/MG90S remain unverified.']
        horn['observations'] = [
            {'source': SOURCE, 'note': 'This photographed specimen is distinct from the AU TA0132 drawing. Double has6 openings per arm; single has7. Paired off-axis dimples are not additional through holes.'},
            {'source': SOURCE, 'note': 'Original pixel anchors, lateral hub-span calibration, independent axial normalization and each derived parameter are retained in estimates. No physical error bound is assigned.'},
            {'source': SOURCE, 'note': 'The local2.43 label cannot be used as a common pitch with all visible openings and the reported overall length. Single21.3 minus hub6.76 gives14.54 exposed reach, differing from printed14.3 by0.24. These inconsistencies remain explicit rather than silently merging dimensions.'}]
        horn['unknown'] = ['Exact manufacturer, SKU, physical revision and particular SG90 compatibility.',
            'True spline tooth count/profile, major/minor dimensions, axial stop and mating fit.',
            'Physical uncertainty of the manual photo measurements and perspective-derived contour/centers.',
            'Cosmetic molding dimple depths, edge fillets/draft, true cup floor and installed seating height.']
    path.write_text(json.dumps(pack, indent=2) + '\n')


if __name__ == '__main__':
    rebuild()
