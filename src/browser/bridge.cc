#include "carbonyl/src/browser/bridge.h"

#include <cmath>

#include "carbonyl/src/browser/bridge_state.h"
#include "content/public/browser/host_zoom_map.h"
#include "content/public/browser/render_frame_host.h"
#include "content/public/browser/web_contents.h"

namespace {

float default_zoom_factor_ = 1.0f;
content::WebContents* web_contents_ = nullptr;

constexpr double kZoomFactorIncrement = 1.2;

double ZoomFactorToZoomLevel(float factor) {
    return std::log(static_cast<double>(factor)) /
           std::log(kZoomFactorIncrement);
}

}

namespace carbonyl {

void Bridge::Resize() {}

float Bridge::GetDPI() {
    return bridge::GetDPI();
}

float Bridge::GetDeviceScaleFactor() {
    return bridge::GetDeviceScaleFactor();
}

bool Bridge::BitmapMode() {
    return bridge::BitmapMode();
}

void Bridge::SetDeviceScaleFactor(float dsf) {
    if (dsf < 1.0f) {
        dsf = 1.0f;
    } else if (dsf > 3.0f) {
        dsf = 3.0f;
    }

    bridge::SetDeviceScaleFactor(dsf);
    bridge::SetDPI(dsf);
}

void Bridge::SetDefaultZoom(float factor) {
    if (factor < 0.1f) {
        factor = 0.1f;
    }

    default_zoom_factor_ = factor;

    if (!web_contents_) {
        return;
    }

    auto* host_zoom_map =
        content::HostZoomMap::GetForWebContents(web_contents_);

    if (!host_zoom_map) {
        return;
    }

    const double zoom_level = ZoomFactorToZoomLevel(default_zoom_factor_);

    if (auto* main_frame = web_contents_->GetPrimaryMainFrame()) {
        host_zoom_map->SetZoomLevel(main_frame->GetGlobalId(), zoom_level);
    }

    host_zoom_map->SetDefaultZoomLevel(zoom_level);
}

void Bridge::SetWebContents(content::WebContents* web_contents) {
    web_contents_ = web_contents;

    if (!web_contents_) {
        return;
    }

    Bridge::SetDefaultZoom(default_zoom_factor_);
}

void Bridge::Configure(float dpi) {
    bridge::SetBitmapMode(false);
    Bridge::SetDeviceScaleFactor(dpi);
}

}

extern "C" void carbonyl_set_device_scale_factor(float dsf) {
    carbonyl::Bridge::SetDeviceScaleFactor(dsf);
}

extern "C" void carbonyl_set_default_zoom(float factor) {
    carbonyl::Bridge::SetDefaultZoom(factor);
}
