use glifparser::glif::{MFEKContour, MFEKOutline, PAPContour};
use MFEKmath::{pattern_along_path_mfek, Piecewise};

use super::ContourOperationData;
use crate::util::MFEKGlifPointData;

impl ContourOperationData for PAPContour<MFEKGlifPointData> {
    fn build(&self, contour: &MFEKContour<MFEKGlifPointData>) -> MFEKOutline<MFEKGlifPointData> {
        let contour_pw = Piecewise::from(&contour.inner);

        let pap_output = pattern_along_path_mfek(&contour_pw, self);

        let mut output: MFEKOutline<MFEKGlifPointData> = Vec::new();
        for contour in pap_output.segs {
            output.push(contour.to_contour().into());
        }

        output
    }

    fn sub(&mut self, _contour: &MFEKContour<MFEKGlifPointData>, _begin: usize, _end: usize) { }

    fn append(
        &mut self,
        _contour: &MFEKContour<MFEKGlifPointData>,
        _append: &MFEKContour<MFEKGlifPointData>,
    ) { }

    fn insert(&mut self, _contour: &MFEKContour<MFEKGlifPointData>, _point_idx: usize) { }
}
