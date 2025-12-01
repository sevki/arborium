import std.stdio;
import std.algorithm;

struct Point {
    double x, y;

    double magnitude() const {
        return (x * x + y * y) ^^ 0.5;
    }
}

void main() {
    auto points = [Point(3, 4), Point(1, 1)];
    auto mags = points.map!(p => p.magnitude);

    foreach (m; mags) {
        writefln("Magnitude: %.2f", m);
    }
}
