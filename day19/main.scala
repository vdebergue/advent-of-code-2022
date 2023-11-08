//> using scala "3"
import scala.io.Source

@main def main(args: String*): Unit =
  val input = Source.fromFile(args.head).mkString
  val t1 = part1(input)
  println(s"Result is $t1")

  // val t2 = part2(input)
  // println(s"Result is $t2")

def part1(input: String): Int =
  ???

case class State(
    stock: Resources,
    robots: Resources,
    time: Int
)
object State {
  def init: State = State(
    stock = Resources(),
    robots = Resources().copy(ores = 1),
    time = 24
  )

  def collectResources(): State = State(
    stock = stock + robots,
    robots = robots,
    time = time - 1
  )
}

case class Resources(
    ores: Int = 0,
    clays: Int = 0,
    obsidians: Int = 0,
    geodes: Int = 0
) {
  def +(other: Resources): Resources = Resources(
    ores = ores + other.ores,
    clays = clays + other.clays,
    obsidians = obsidians + other.obsidians,
    geodes = geodes + other.geodes
  )
  def -(other: Resources): Resources = Resources(
    ores = ores - other.ores,
    clays = clays - other.clays,
    obsidians = obsidians - other.obsidians,
    geodes = geodes - other.geodes
  )
}
case class Blueprint(
    oreRobotCost: Resources,
    clayRobotCost: Resources,
    obsidianRobotCost: Resources,
    geodeRobotCost: Resources
)

def search(blueprint: Blueprint, cache: Map[State, Int], state: State): Int =
  if (state.time == 0) return state.stock.geodes
  if (cache.contains(state)) return cache.get(state).get

  val maxVal = state.stock.geodes + state.robots.geodes * state.time
