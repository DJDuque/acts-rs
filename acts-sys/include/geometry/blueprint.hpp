#pragma once

#include "Acts/Geometry/Blueprint.hpp"
#include "Acts/Geometry/Extent.hpp"
#include <memory>

namespace acts_sys {
namespace ffi {

using BlueprintConfig = Acts::Experimental::Blueprint::Config;

inline std::unique_ptr<BlueprintConfig>
new_blueprint_config(const Acts::ExtentEnvelope &envelope) {
  auto cfg = std::make_unique<BlueprintConfig>();
  cfg->envelope = envelope;
  return cfg;
}

inline Acts::Experimental::Blueprint &
blueprint_add_child(Acts::Experimental::Blueprint &node,
                    std::shared_ptr<Acts::Experimental::BlueprintNode> child) {
  return static_cast<Acts::Experimental::Blueprint &>(
      node.addChild(std::move(child)));
}

} // namespace ffi
} // namespace acts_sys
