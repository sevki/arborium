module Example

struct Point{T<:Number}
    x::T
    y::T
end

function magnitude(p::Point)
    sqrt(p.x^2 + p.y^2)
end

# Multiple dispatch
Base.:+(a::Point, b::Point) = Point(a.x + b.x, a.y + b.y)

points = [Point(3.0, 4.0), Point(1.0, 1.0)]
mags = [magnitude(p) for p in points]
println("Magnitudes: $mags")

end
