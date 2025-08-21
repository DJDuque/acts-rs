#pragma once

#include "Acts/Visualization/ViewConfig.hpp"
#include <memory>

namespace acts_sys {
namespace ffi {

inline std::unique_ptr<Acts::ViewConfig>
new_view_config(bool visible, const std::array<int, 3> &rgb_color,
                double offset, double line_thickness, double surface_thickness,
                unsigned int quarter_segments, bool triangulate,
                const std::string &output_name) {
  auto cfg = std::make_unique<Acts::ViewConfig>();
  cfg->visible = visible;
  cfg->color = Acts::Color(rgb_color);
  cfg->offset = offset;
  cfg->lineThickness = line_thickness;
  cfg->surfaceThickness = surface_thickness;
  cfg->quarterSegments = quarter_segments;
  cfg->triangulate = triangulate;
  cfg->outputName = output_name;

  return cfg;
}

} // namespace ffi
} // namespace acts_sys
