use opencv::{highgui, videoio, prelude::*, core::{Scalar, Vector, Point2f},};
fn contours_matching() -> Result<(), String> {
    let name = "test";
    highgui::named_window(name, highgui::WINDOW_GUI_NORMAL | highgui::WINDOW_KEEPRATIO | highgui::WINDOW_AUTOSIZE)
        .map_err(|err| format!("[main] failed to create window: {err}"))?;
    let mut capture = videoio::VideoCapture::new(0, videoio::CAP_ANY)
        .map_err(|err| format!("[main] failed to capture video from  main camera: {err}"))?;
    capture.set(videoio::CAP_PROP_FRAME_WIDTH, 1920.0)
        .map_err(|err| format!("[main] failed to set capture width: {err}"))?;
    capture.set(videoio::CAP_PROP_FRAME_HEIGHT, 1080.0)
        .map_err(|err| format!("[main] failed to set capture height: {err}"))?;
    let mut frame = Mat::default();
    let dictionary = opencv::objdetect::get_predefined_dictionary(opencv::objdetect::PredefinedDictionaryType::DICT_4X4_1000).unwrap();
    let mut corners = Vector::<Vector<Point2f>>::default();
    let mut rejected = Vector::<Vector<Point2f>>::default();
    let mut ids = Vector::<i32>::default();
    let detector_params = opencv::objdetect::DetectorParameters::default()
        .map_err(|err| format!("[main] failed to get default DetectorParameters: {err}"))?;
    let refine_params = opencv::objdetect::RefineParameters::new(10.0, 3.0, true)
        .map_err(|err| format!("[main] failed to get default RefineParameters: {err}"))?;
    loop {
        capture.read(&mut frame)
        .map_err(|err| format!("[main] failed to read a frame from VideoCaprute: {err}"))?;
        let detector = opencv::objdetect::ArucoDetector::new(&dictionary, &detector_params, refine_params)
            .map_err(|err: opencv::Error| format!("[main] ArucoDetector.new: {err}"))?;
        detector.detect_markers(&frame, &mut corners, & mut ids, &mut rejected)
            .map_err(|err: opencv::Error| format!("[main] detect_markers: {err}"))?;
        opencv::objdetect::draw_detected_markers(&mut frame, &corners, &ids, Scalar::new(0.0, 255.0, 0.0, 255.0))
            .map_err(|err: opencv::Error| format!("[main] draw_detected_markers: {err}"))?;
        highgui::imshow(name, &frame)
            .map_err(|err| format!("[main] failed to display an image in the window: {err}"))?;
        let key = highgui::wait_key(1)
            .map_err(|err| format!("[main] highgui::wait_key error: {err}"))?;
        if key == 'q' as i32 {
            break;
        }
    }
    highgui::destroy_window(name)
        .map_err(|err| format!("[main] failed to destroy window: {err}"))?;
    Ok(())
}
fn main() -> Result<(), String> {
    contours_matching()
}
