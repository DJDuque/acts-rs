#pragma once

#include "Acts/Definitions/Direction.hpp"
#include <memory>

namespace acts_sys {
namespace ffi {

	std::unique_ptr<Acts::Direction> new_negative_direction() {
		return std::make_unique<Acts::Direction>(Acts::Direction::Negative());
	}

	std::unique_ptr<Acts::Direction> new_positive_direction() {
		return std::make_unique<Acts::Direction>(Acts::Direction::Positive());
	}

}
}
