use MFEKmath::{Bezier, Primitive};
use glifparser::{glif::{contour::MFEKContourCommon, contour_operations::ContourOperation, point::quad::QPoint, MFEKContour, inner::MFEKContourInner}, Point, PointType, WhichHandle, contour::State, Contour, MFEKPointData, Handle};
use glifrenderer::points::draw_point;

use crate::{editor::{Editor, util::{HoveredPointInfo}}, user_interface::MouseInfo, get_contour_len, tool_behaviors::move_handle::MoveHandle};
use super::PenMode;

/// This is the cubic specific pen implementation, and should serve as an example of the new pen API.
/// One of the biggest things of note is that these mode structs should -not- contain modification specific state.
/// State like that should be handled purely inside ToolBehaviors implemented for the mode. You can have state in these
/// top level modes like UI state where a user might select a point type e.g. Spiro, but these modes are instantiated with
/// Pen so the state will carry across modifications, and mode changes.
#[derive(Clone, Debug)]
pub struct QuadMode {
    // cubic requires no state unlike a mode like Spiro
}

impl PenMode for QuadMode {
    fn new_contour(&self, v: &mut Editor, mouse_info: MouseInfo) {
        let mouse_pos = mouse_info.position;
        v.contour_idx = {
            let layer = v.get_active_layer_mut();
            let mut new_contour = Vec::new();
            
            let ptype = if mouse_info.modifiers.shift {
                PointType::Curve
            } else {
                PointType::Move
            };

            let (x, y) = mouse_pos;
            
            new_contour.push(QPoint {
                x, y,
                a: Handle::Colocated,
                name: None,
                ptype,
                smooth: false,
                data: None,
            });

            let contour_outer: MFEKContour<MFEKPointData> = MFEKContour {
                inner: MFEKContourInner::Quad(new_contour),
                operation: None,
            };

            layer.outline.push(contour_outer);
            Some(layer.outline.len() - 1)
        };
        v.point_idx = Some(0);
    }
    
    fn add_point(&self, v: &mut Editor, mouse_info: MouseInfo) {
        let mouse_pos = mouse_info.position;
        let contour_idx = v.contour_idx.unwrap();
        let contour_len = get_contour_len!(v.get_active_layer_ref(), contour_idx);

        if v.point_idx.unwrap() == contour_len - 1 {
            v.point_idx = {
                let layer = v.get_active_layer_mut();
                layer.outline[contour_idx].operation.insert_op(contour_len);
                let contour = layer.outline[contour_idx].quad_mut().unwrap();
                contour.push(QPoint::from_x_y_type(
                    (mouse_pos.0 as f32, mouse_pos.1 as f32),
                    PointType::Curve,
                ));

                Some(get_contour_len!(layer, contour_idx) - 1)
            };
        } else if v.point_idx.unwrap() == 0 {
            {
                let layer = v.get_active_layer_mut();
                let contour = layer.outline[contour_idx].quad_mut().unwrap();
                let point_type = contour[0].ptype;

                if contour.is_open() {
                    contour[0].ptype = PointType::Curve;
                }

                contour.insert(
                    0,
                    QPoint::from_x_y_type((mouse_pos.0 as f32, mouse_pos.1 as f32), point_type),
                );

                layer.outline[contour_idx].operation.insert_op( 0);
            };
            v.end_modification();
        }
        v.push_behavior(Box::new(MoveHandle::new(WhichHandle::A, mouse_info, true)));
    }

    // TODO: Implement these for quadratic! Would take a bit more work in math.rlib
    // These functions are safe to be stubbed for now
    fn draw_nearest_point(&self, i: &crate::user_interface::Interface, canvas: &mut MFEKmath::skia_safe::Canvas, info: HoveredPointInfo) {}
    fn subdivide_curve(&self, v: &mut Editor, info: HoveredPointInfo) {}
}