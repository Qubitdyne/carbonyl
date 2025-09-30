#include "carbonyl/src/browser/bridge_state.h"

namespace carbonyl {
namespace bridge {
namespace {

bool bitmap_mode = false;
float device_scale_factor = 1.0f;
float dpi = 0.0f;

}  // namespace

bool BitmapMode() {
  return bitmap_mode;
}

void SetBitmapMode(bool bitmap_mode_value) {
  bitmap_mode = bitmap_mode_value;
}

float GetDeviceScaleFactor() {
  return device_scale_factor;
}

void SetDeviceScaleFactor(float device_scale_factor_value) {
  device_scale_factor = device_scale_factor_value;
}

float GetDPI() {
  return dpi;
}

void SetDPI(float dpi_value) {
  dpi = dpi_value;
}

}  // namespace bridge
}  // namespace carbonyl
