#ifndef CARBONYL_SRC_BROWSER_BRIDGE_H_
#define CARBONYL_SRC_BROWSER_BRIDGE_H_

#include "carbonyl/src/browser/export.h"

namespace content {
class WebContents;
}

namespace carbonyl {

class Renderer;

class CARBONYL_BRIDGE_EXPORT Bridge {
public:
  static float GetDPI();
  static bool BitmapMode();
  static float GetDeviceScaleFactor();
  static void SetDeviceScaleFactor(float dsf);
  static void SetDefaultZoom(float factor);
  static void SetWebContents(content::WebContents* web_contents);

private:
  friend class Renderer;

  static void Resize();
  static void Configure(float dpi);
};

}

extern "C" {
CARBONYL_BRIDGE_EXPORT void carbonyl_set_device_scale_factor(float dsf);
CARBONYL_BRIDGE_EXPORT void carbonyl_set_default_zoom(float factor);
}

#endif  // CARBONYL_SRC_BROWSER_BRIDGE_H_
