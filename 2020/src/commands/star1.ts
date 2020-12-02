import {Command} from '@oclif/command'
import {readFileSync} from 'fs'
export default class Star1 extends Command {
  static description = 'Star 1'

  static examples = [
    `$ aoc star1
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
  // TODO: extract reading input into a library


    async run() {
      const input = readFileSync('inputs/day1.txt').toString('utf-8')
      const parsed = input.split('\n').map(Number)
      for (const a of parsed) {
        const b = parsed.indexOf(2020 - Number(a))
        if (b > -1) {
          const answer = a * parsed[b]
          this.log(String(answer))
          break
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
