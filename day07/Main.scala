//> using scala "3"
import scala.io.Source
import scala.io.BufferedSource

def input = Source.fromFile("input")

def test = """$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"""

@main
def main(): Unit =
  println("Hello world")
  val rootTest = readFs(test.linesIterator)
  val resTest  = part1(rootTest)
  println(s"Test => ${resTest}")
  println(rootTest)

  val root = readFs(input.getLines())
  val res1 = part1(root)
  println(s"Res1 => ${res1}")
  println(s"Res2 => ${part2(root)}")

def part1(root: Dir): Int = {
  val totals  = allDirs(root).map(_.totalSize())
  val lowSize = totals.filter(_ <= 100_000)
  println(s"lowSize= ${lowSize.size}")
  lowSize.sum
}

def part2(root: Dir): Int = {
  val totalSize = 70000000
  val free      = totalSize - root.totalSize()
  val need      = 30000000
  val toFree    = need - free
  val totals    = allDirs(root).map(_.totalSize())
  totals.filter(_ > toFree).min
}

def allDirs(root: Dir): List[Dir] = {
  def loop(n: Dir): List[Dir] = n :: n.dirs.values.flatMap(loop).toList
  loop(root)
}

def readFs(content: Iterator[String]): Dir = {
  val root    = Dir(name = "/", parent = None)
  var current = root

  content.foreach { line =>
    val parts = line.split(" ")
    if (parts(0) == "$") {
      val command = parts(1)
      command match
        case "cd" =>
          val path = parts(2)
          if (path == "..") {
            current = current.parent.get
          } else if (path == "/") {
            current = root
          } else {
            val node = current.dirs.get(path) match {
              case Some(n) => n
              case None =>
                println(s"Adding ${path} to ${current.name}")
                val n = Dir(path, parent = Some(current))
                current.dirs.update(path, n)
                n
            }
            current = node
          }
        case "ls" =>
        // nothing
    } else {
      if (parts(0) == "dir") {
        // skip
      } else {
        val size = parts(0).toInt
        val name = parts(1)
        val node = File(name, size)
        println(s"Adding File ${name} to ${current.name}")
        current.files.append(node)
      }
    }
  }
  root
}

case class File(name: String, size: Int)
case class Dir(
    name: String,
    parent: Option[Dir],
    dirs: collection.mutable.Map[String, Dir] = collection.mutable.Map.empty,
    files: collection.mutable.ListBuffer[File] = collection.mutable.ListBuffer.empty
) {
  def totalSize(): Int = {
    var size = this.files.map(_.size).sum
    this.dirs.values.foreach(d => size = size + d.totalSize())
    size
  }
  override def toString(): String = s"Dir($name, $files, $dirs)"
}
