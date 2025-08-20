#pragma once

#include "Acts/Geometry/LayerBlueprintNode.hpp"
#include "Acts/Surfaces/Surface.hpp"
#include <memory>

namespace acts_sys {
namespace ffi {

using LayerBlueprintNodeLayerType =
    Acts::Experimental::LayerBlueprintNode::LayerType;
using SharedSurface = std::shared_ptr<Acts::Surface>;

// This is needed because we can't pass CxxVector by value through FFI bridge
inline void
layer_blueprint_node_set_surfaces(Acts::Experimental::LayerBlueprintNode &self,
                                  const std::vector<SharedSurface> &surfaces) {
  self.setSurfaces(surfaces);
}

} // namespace ffi
} // namespace acts_sys
