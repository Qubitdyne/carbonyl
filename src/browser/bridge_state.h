#ifndef CARBONYL_SRC_BROWSER_BRIDGE_STATE_H_
#define CARBONYL_SRC_BROWSER_BRIDGE_STATE_H_

namespace carbonyl {
namespace bridge {

bool BitmapMode();
void SetBitmapMode(bool bitmap_mode);

float GetDeviceScaleFactor();
void SetDeviceScaleFactor(float device_scale_factor);

float GetDPI();
void SetDPI(float dpi);

}  // namespace bridge
}  // namespace carbonyl

#endif  // CARBONYL_SRC_BROWSER_BRIDGE_STATE_H_
