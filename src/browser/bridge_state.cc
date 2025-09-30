#include "carbonyl/src/browser/bridge_state.h"

namespace carbonyl {
namespace bridge {
namespace {

float device_scale_factor = 1.0f;
float dpi = 0.0f;

}  // namespace

bool BitmapMode() {
  return false;
}

void SetBitmapMode(bool bitmap_mode_value) {
  (void)bitmap_mode_value;
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
