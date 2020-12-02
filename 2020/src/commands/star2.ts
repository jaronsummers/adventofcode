import {Command} from '@oclif/command'
import {readFileSync} from 'fs'
export default class Star2 extends Command {
  static description = 'Star 2'

  static examples = [
    `$ aoc star2
hello world from ./src/hello.ts!
`,
  ]

  // static flags = {
  //   help: flags.help({char: 'h'}),
  //   // flag with a value (-n, --name=VALUE)
  //   name: flags.string({char: 'n', description: 'name to print'}),
  //   // flag with no value (-f, --force)
  //   force: flags.boolean({char: 'f'}),
  // }
  //
  // static args = [{name: 'file'}]

  async run() {
    // I haven't put a lot of thought into how to optimize this, but it may be useful to keep track of the small numbers?
    const input = readFileSync('inputs/day1.txt').toString('utf-8')
    const parsed = input.split('\n').filter(n => n).map(Number)
    // const smallest = parsed[parsed.indexOf(Math.min.apply(null, parsed))]
    for (const a of parsed) {
      // lol
      for (const b of parsed.slice(parsed.indexOf(a))) {
        const c = parsed.indexOf(2020 - a - b)
        if (c > -1) {
          const answer = a * b * parsed[c]
          // this prints twice for some reason, I must not understand how break works
          this.log(String(answer))
          break
        }
      }
    }
    // const {args, flags} = this.parse(Hello)

    // const name = flags.name ?? 'world'
    // this.log(`hello ${name} from .\\src\\commands\\star1.ts`)
    // if (args.file && flags.force) {
    //   this.log(`you input --force and --file: ${args.file}`)
    // }
  }
}
