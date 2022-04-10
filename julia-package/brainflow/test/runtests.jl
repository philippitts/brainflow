#!/usr/bin/env julia
using BrainFlow

include("julia_tests.jl")
include("../examples/brainflow_get_data.jl")
include("../examples/brainflow_get_data_twice.jl")
include("../examples/signal_filtering.jl")
include("../examples/denoising.jl")
include("../examples/downsampling.jl")
include("../examples/csp.jl")
include("../examples/serialization.jl")
include("../examples/transforms.jl")
include("../examples/eeg_metrics.jl")
include("../examples/markers.jl")
include("../examples/release_all.jl")
