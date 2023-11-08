//> using scala "3"
import scala.io.Source

@main def main(args: String*): Unit =
    val input = Source.fromFile(args.head).mkString
    val t1 = part1(input)
    println(s"Result is $t1")

    val t2 = part2(input)
    println(s"Result is $t2")

def part1(input: String): Int = 
    val state = State.parse(input)
    state.cubes.map { cube => 
        cube.sides.count(s => state.cubes.forall(c => c != s))
    }.sum


def part2(input: String): Int = 
    val state = State.parse(input)
    val inside = collection.mutable.Set.empty[Cube]
    val outside = collection.mutable.Set.empty[Cube]

    def reachesOutside(cube: Cube): Boolean =
        if (outside.contains(cube)) true
        else if (inside.contains(cube)) false 
        else {
            val seen = collection.mutable.Set.empty[Cube]
            var queue = cube :: Nil
            while (queue != Nil) {
                val elem = queue.head
                queue = queue.tail
                if (state.cubes.contains(elem)) {}
                else if (seen.contains(elem)) {}
                else {
                    seen += elem
                    if (seen.size >= 5000) {
                        seen.foreach(c => outside += c)
                        return true
                    } else {
                        queue = queue ++ elem.sides.toList
                    }
                }
            }
            seen.foreach(c => inside += c)
            false
        }

    state.cubes.map { cube =>
        cube.sides.count{side => 
            reachesOutside(side)
        }
    }.sum

case class State(cubes: Seq[Cube])

object State {
    def parse(input: String): State = {
        val cubes = input.linesIterator
        .filter(_.nonEmpty)
        .map { line =>
            val Array(x,y,z) = line.split(',').map(_.toInt)
            Cube(x,y,z)
        }.toIndexedSeq
        State(cubes)
    }
}

case class Cube(x: Int, y: Int, z: Int) {
    def sides: Seq[Cube] = 
        Seq(
            Cube(x +1, y, z),
            Cube(x -1, y, z),
            Cube(x, y +1, z),
            Cube(x, y -1, z),
            Cube(x, y, z + 1),
            Cube(x, y, z -1),
        )
}
