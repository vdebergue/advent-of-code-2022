//> using scala "3"
import scala.io.Source

def input = Source.fromFile("input")

def test = """
30373
25512
65332
33549
35390
""".split("\n").filter(_.nonEmpty)

@main
def main() =
    val forestTest = parse(test.iterator)
    setVisibility(forestTest)
    println(forestTest.mkString("\n"))
    println(s"Visible => ${countVisible(forestTest)}")
    scenicScore(forestTest)
    println(forestTest.mkString("\n"))
    println(s"score => ${maxScenicScore(forestTest)}")

    val forest = parse(input.getLines())
    setVisibility(forest)
    scenicScore(forest)
    println(s"Visible => ${countVisible(forest)}")
    println(s"score => ${maxScenicScore(forest)}")

def parse(lines: Iterator[String]): Vector[Vector[Tree]] =
    lines.map { line =>
        line.map(c => Tree(c.toString().toInt)).toVector
    }.toVector

def setVisibility(forest: Vector[Vector[Tree]]) = 
    val width = forest(0).length
    val height = forest.length

    def walk(dir: Direction): IndexedSeq[IndexedSeq[Tree]] = 
        dir match
            case Direction.Top => 
                for (i <- 0 until width) yield {
                    for (j <- 0 until height) yield forest(i)(j)
                }
            case Direction.Bottom => 
                for (i <- 0 until width) yield {
                    for (j <- (0 until height).reverse) yield forest(i)(j)
                }
            case Direction.Left => 
                for (j <- 0 until height) yield {
                    for (i <- 0 until width) yield forest(i)(j)
                }
            case Direction.Right => 
                for (j <- 0 until height) yield {
                    for (i <- (0 until width).reverse) yield forest(i)(j)
                }

    def mark(dir: Direction) = 
        val trees = walk(dir)
        for (lineOfTree <- trees) {
            var current = -1
            for (tree <- lineOfTree) {
                if (tree.height > current) {
                    tree.visible = true
                    current = tree.height
                }
            }
        }

    Direction.values.foreach(mark)

def scenicScore(forest: Vector[Vector[Tree]]) = 
    val width = forest(0).length
    val height = forest.length

    def walk(from: (Int, Int), dir: Direction): IndexedSeq[Tree] =
        val (y, x) = from
        dir match
            case Direction.Top => 
                for (i <- (0 until y).reverse) yield forest(i)(x)
            case Direction.Bottom =>
                for (i <- (y + 1 until height)) yield forest(i)(x)
            case Direction.Right =>
                for (i <- (x + 1 until width)) yield forest(y)(i)
            case Direction.Left =>
                for (i <- (0 until x).reverse) yield forest(y)(i)

    for (i <- 0 until height) {
        for (j <- 0 until width) {
            val tree = forest(i)(j)
            // println(s"For tree ($j, $i) ${tree.height}}")
            val score = Direction.values.map { dir =>
                val trees = walk((i,j), dir)
                val criterion = (t: Tree) => t.height < tree.height
                val score = (trees.takeWhile(criterion) ++ trees.dropWhile(criterion).take(1)).size
                // println(s"  $dir => score=$score | $trees")
                score
            }.fold(1)(_ * _)
            // println(s" score = ${score}")
            tree.score = score
        }
    }

def countVisible(forest: Vector[Vector[Tree]]) = 
    forest.map(_.count(_.visible)).sum

def maxScenicScore(forest: Vector[Vector[Tree]]): Int = 
    forest.map(_.map(_.score).max).max


case class Tree(height: Int, var visible: Boolean = false, var score: Int = 0)

enum Direction:
    case Top, Bottom, Right, Left
