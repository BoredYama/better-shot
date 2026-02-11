//! OCR module using macOS Vision framework

use crate::utils::AppResult;

#[cfg(target_os = "macos")]
pub fn recognize_text_from_image(image_path: &str) -> AppResult<String> {
    use objc2::rc::autoreleasepool;
    use objc2::runtime::AnyObject;
    use objc2::AnyThread;
    use objc2_foundation::{NSArray, NSDictionary, NSString, NSURL};
    use objc2_vision::{
        VNImageRequestHandler, VNRecognizeTextRequest, VNRecognizedTextObservation,
        VNRecognizedText, VNRequest, VNRequestTextRecognitionLevel,
    };
    use std::path::Path;

    let path = Path::new(image_path);
    if !path.exists() {
        return Err(format!("Image file does not exist: {}", image_path));
    }

    autoreleasepool(|_| {
        unsafe {
            let ns_string = NSString::from_str(image_path);
            let ns_url = NSURL::fileURLWithPath_isDirectory(&ns_string, false);
            let options = NSDictionary::<NSString, AnyObject>::new();

            let handler = VNImageRequestHandler::initWithURL_options(
                VNImageRequestHandler::alloc(),
                &ns_url,
                &*options,
            );

            let text_request = VNRecognizeTextRequest::init(
                VNRecognizeTextRequest::alloc(),
            );

            text_request.setRecognitionLevel(VNRequestTextRecognitionLevel::Accurate);
            text_request.setUsesLanguageCorrection(true);

            let request_ref: &VNRequest = text_request.as_ref();
            let requests = NSArray::from_slice(&[request_ref]);
            
            handler
                .performRequests_error(&requests)
                .map_err(|e| format!("Vision request failed: {:?}", e))?;

            let observations = text_request.results();
            let mut recognized_texts = Vec::new();

            if let Some(obs_array) = observations {
                for obs in obs_array.iter() {
                    if let Some(text_obs) = obs.downcast_ref::<VNRecognizedTextObservation>() {
                        let candidates = text_obs.topCandidates(1);
                        for cand in candidates.iter() {
                            if let Some(text_cand) = cand.downcast_ref::<VNRecognizedText>() {
                                let str_ref = text_cand.string();
                                recognized_texts.push(str_ref.to_string());
                            }
                        }
                    }
                }
            }

            if recognized_texts.is_empty() {
                return Err("No text recognized in image".to_string());
            }

            Ok(recognized_texts.join("\n"))
        }
    })
}

#[cfg(not(target_os = "macos"))]
pub fn recognize_text_from_image(image_path: &str) -> AppResult<String> {
    use std::process::Command;

    // Check if tesseract is installed
    if Command::new("tesseract").arg("--version").output().is_err() {
        return Err("Tesseract OCR is not installed. Please install 'tesseract-ocr' (and language data) to use this feature.".to_string());
    }

    let output = Command::new("tesseract")
        .arg(image_path)
        .arg("stdout") // print to stdout
        .output()
        .map_err(|e| format!("Failed to execute tesseract: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Tesseract execution failed: {}", stderr));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
