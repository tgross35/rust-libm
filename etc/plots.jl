#!/usr/bin/env -S julia -t 4 --project

"A quick script for plotting a list of floats.

Takes a path to a TOML file (Julia has builtin TOML support but not JSON) which
specifies a list of source files to plot. Plots are done with both a linear and
a log scale.

Requires [Makie] (specifically CairoMakie) for plotting.

[Makie]: https://docs.makie.org/stable/
"


using CairoMakie
using MathTeXEngine
using SpecialFunctions

function main()
    out_dir = joinpath(@__DIR__, "plots")

    pi2ticks = pi_ticks(2)
    pi4ticks = pi_ticks(4)

    plot_unary(out_dir, sin, (-2pi, 2pi), (-1.2, 1.2), xticks = pi2ticks)
    plot_unary(out_dir, cos, (-2pi, 2pi), (-1.2, 1.2), xticks = pi2ticks)
    plot_unary(out_dir, tan, (-2pi, 2pi), (-4.0, 4.0), xticks = pi2ticks)

    plot_unary(out_dir, asin, (-1.5, 1.5), (-pi / 2 - 0.2, pi / 2 + 0.2), yticks = pi4ticks)
    plot_unary(out_dir, acos, (-1.5, 1.5), (-0.2, pi + 0.2), yticks = pi4ticks)
    plot_unary(
        out_dir,
        atan,
        (-5.0, 5.0),
        (-pi / 2 - 0.2, pi / 2 + 0.2),
        xticks = LinearTicks(4), # default only produces two ticks
        yticks = pi4ticks,
    )

    plot_unary(out_dir, sinh, (-4.0, 4.0), (-8.0, 8.0))
    plot_unary(out_dir, cosh, (-4.0, 4.0), (-0.5, 5.5))
    plot_unary(out_dir, tanh, (-4.0, 4.0), (-1.2, 1.2))

    plot_unary(out_dir, asinh, (-8.0, 8.0), (-3.0, 3.0))
    plot_unary(out_dir, acosh, (-0.2, 6.0), (-0.2, 3.0))
    # TODO asymptotes clipped
    plot_unary(out_dir, atanh, (-1.5, 1.5), (-4.0, 4.0))


    plot_unary(out_dir, abs, (-5.0, 5.0), (-0.2, 5.2), name = "fabs")
    plot_unary(out_dir, ceil, (-4.0, 4.0), (-4.0, 4.0))
    plot_unary(out_dir, floor, (-4.0, 4.0), (-4.0, 4.0))
    plot_unary(out_dir, trunc, (-4.0, 4.0), (-4.0, 4.0))
    # plot_unary(out_dir, rint, (-5.0, 5.0), (-1.2, 1.2))
    # plot_unary(out_dir, round, (-5.0, 5.0), (-1.2, 1.2))

    plot_unary(out_dir, sqrt, (-0.2, 6.0), (-0.2, 3.0))
    plot_unary(out_dir, cbrt, (-6.0, 6.0), (-2.0, 2.0))

    plot_unary(
        out_dir,
        "exponentials",
        "Exponentials",
        [
            Series(exp, L"e^x", Dict(:space => :relative)),
            Series(exp10, L"10^x", Dict(:space => :relative)),
            Series(exp2, L"2^x", Dict(:space => :relative)),
            Series(expm1, L"e^x-1", Dict(:space => :relative)),
        ],
        (-4.0, 4.0),
        (-1.2, 10.0),
        legend_position = :lt,
    )

    plot_unary(
        out_dir,
        "logarithms",
        "Logarithms",
        [
            Series(log, L"\ln(x)"),
            Series(log10, L"\log_{10}(x)"),
            Series(log1p, L"\log(1 + x)"),
            Series(log2, L"\log_2(x)"),
        ],
        (-1.2, 10.0),
        (-2.5, 4.0),
        legend_position = :rb,
    )

    plot_unary(out_dir, erf, (-3.5, 3.5), (-1.2, 1.2), title = L"\text{erf} x")
    plot_unary(out_dir, erfc, (-3.5, 3.5), (-0.2, 2.2), title = L"\text{erfc} x")

    plot_unary(
        out_dir,
        gamma,
        (-5.0, 5.0),
        (-5.0, 5.0),
        name = "tgamma",
        title = L"y = \Gamma(z)",
        xlabel = L"\Re(z)",
    )
    plot_unary(
        out_dir,
        lgamma,
        (0.0, 8.0),
        (-1.2, 8.0),
        title = L"y = \ln \Gamma(z)",
        xlabel = L"\Re(z)",
    )

    plot_unary(
        out_dir,
        "bessel",
        "Bessel Functions",
        [
            Series(besselj0, L"J_0(x)"),
            Series(besselj1, L"J_1(x)"),
            Series(x -> besselj(2, x), L"J_2(x)"),
        ],
        (-15.0, 15.0),
        (-0.8, 1.2),
        legend_position = :rb,
    )

    plot_unary(
        out_dir,
        "bessel-second",
        "Second Order Bessel Functions",
        [
            Series(bessely0, L"Y_0(x)"),
            Series(bessely1, L"Y_1(x)"),
            Series(x -> bessely(2, x), L"Y_2(x)"),
        ],
        (-1.0, 25.0),
        (-2.5, 1.0),
        legend_position = :rb,
        xticks = LinearTicks(8),
    )
end

"One line to plot on an `Axis`."
struct Series
    func::Function
    label::AbstractString
    label_args::Dict
end

Series(func::Function, label::AbstractString) = Series(func, label, Dict())

function plot_unary(
    out_dir::String,
    name::String,
    title::AbstractString,
    series::Vector{Series},
    xlims::Tuple{Number,Number},
    ylims::Union{Tuple{Number,Number},Nothing} = nothing;
    legend_position::Union{Symbol,Nothing} = nothing,
    kwargs...,
)
    fig = Figure(font = texfont(), fontsize = 20)
    out_file = joinpath(out_dir, "$name.svg")
    println("plotting $name")

    ax = Axis(fig[1, 1], title = title, limits = (xlims, ylims); kwargs...)
    vlines!(ax, 0, color = :black)
    hlines!(ax, 0, color = :black)

    x = LinRange(xlims[1], xlims[2], 5000)
    for s in series
        y = @. to_nan(s.func, x)
        fix_discontinuities!(y)
        lines!(ax, x, y, label = s.label, linewidth = 3.0)
    end

    if length(series) > 1
        for s in series
            text!(ax, s.label, space = :relative, offset = (0.5, -0.2))
        end
        axislegend(ax, position = legend_position, framevisible = false)
    end

    save(out_file, fig)
end

"Single-series version of the above."
function plot_unary(
    out_dir::String,
    func::Function,
    xlims::Tuple{Number,Number},
    ylims::Union{Tuple{Number,Number},Nothing} = nothing;
    name::Union{String,Nothing} = nothing,
    title::Union{AbstractString,Nothing} = nothing,
    kwargs...,
)
    if title === nothing
        title = L"\text{%$func}(x)"
    end

    if name === nothing
        name = string(func)
    end

    plot_unary(out_dir, name, title, [Series(func, title)], xlims, ylims; kwargs...)
end

"Turn domain errors into NaN."
function to_nan(f::Function, x)::Float64
    try
        return f(x)
    catch e
        if e isa DomainError
            return NaN
        else
            rethrow(e)
        end
    end
end

"Fix plots like tangent and gamma."
function fix_discontinuities!(v::Vector{Float64})
    for i = 1:(length(v)-1)
        if abs(v[i] - v[1+1]) > 100
            v[i] = NaN
        end
    end
end

"Create evenly spaced ticks at fractions of pi."
function pi_ticks(base::Integer)::Tuple{Vector{Float64},Vector{AbstractString}}
    values = Vector()
    labels = Vector()

    for x = -10:10
        num = numerator(x // base)
        denom = denominator(x // base)
        numstr = abs(num) == 1 ? "" : string(abs(num))
        minus = num < 0 ? "-" : ""
        push!(values, x * pi / base)

        if x == 0
            push!(labels, L"0")
        elseif denom == 1
            push!(labels, L"{%$minus%$numstr}\pi")
        else
            push!(labels, L"{%$minus\frac{%$numstr\pi}{%$denom}}")
        end
    end

    return (values, labels)
end


main()
