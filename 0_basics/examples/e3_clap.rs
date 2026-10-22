#![allow(unused)]

/* 
-------------------------------------------------------------------------------------------------------------------------
CLI ARCHITECTURE

    - foreground application (microservice)
        Cli is the entrypoint to the app, runs in foreground
    - remote client (aws, kubectl, gh)
        runs to completion, but the work happens elsewhere
    - daemon (always running in the background - ollama)
        - cli to control it (start/stop)
        - daemon application itself
        Note: self managing daemon is when the cli and the main application is the same binary. Cli is calling itself with different args.
    - tool
        - runs to completion - cp, grep
        - runs until cancelled - htop, ping
        - attaches to pipeline - jq, sed, sort

-------------------------------------------------------------------------------------------------------------------------
TERMIANL BEHAVIOURS

    Interactive (i/o)
        - check boxes - gh edit repo
        - q&a - aws cli config
    Input
        - free (nano)
        - line by line (kcat -P)
        - REPL (psql, ollama run)
    Output 
        - printing
            - print info
            - print ASCII art
        - dynamic
            - lines (ping, Ctrl+C to stop)
            - window (htop, Ctrl+C to stop)
            - progress bar
        
-------------------------------------------------------------------------------------------------------------------------
CLI DESIGN

0. The canonical invocation shape. The industry has converged on this.

    Single-purpose cli (no subcommands):        program [options]   [positionals]
                                                grep    -i --color  PATTERN FILE
                                                cp      -r          SOURCE DEST

    Multi-purpose cli (subcommands):            program <command> [options] [positionals]
                                                ollama  run                 llama3.2
                                                docker  run       -it       ubuntu

    Note: both [options] and [positionals] can be: required, optional, required-only-if-another-arg-specified, mutually-exclusive
    [options]          - may appear before or after [positionals]                                        
    [subcommand]       - may appear after <command> for complex clis like aws
    [global options]   - may appear after program
    [-- passthrough]   - may appear after [positionals]

1. Positionals
    Identified by where they are, not by name. Can be required or optional. Can be unique or variadic (repeating and must be last). 
    Two positionals is comfortable, three is pushing it, four means you actually wanted named options.
    cp SOURCE DEST
    grep PATTERN FILE

2. Options

    Short - single dash, single letter      -f, -o              Bundleable:     -fo means -f -o
    Long - double dash, whole word          --file, --output    Not bundleable.
    Note: Every flag MUST hava a long form. Only the highest-usage flags need to have a short form also.
    (Legacy tools break the short/long convention. Don't immitate them. E.g. java -version, tar xvf)
    
    Attaching value - you should accept all three ways (libs like clap do this for you):
    --output=file.txt   equals
    --output file.txt   space
    -o file.txt         space

    Negating booleans
    Default for boolean flags is presence=on, absence=off. Negate the flag name if you want the opposite 
    --colour   ->   --no-colour    

    Spaces in the value
    The shell strips the quotes and hands you a space-separated string. It's your cli's job to parse and split on spaces. 
    --input john
    --input "muti word sentence"

    List of values
    -v volume1 -v volume2     Repeated flag. (Best). Unambiguous.
    -t web,api,prod           Comma separated. Compact, but breaks when value contains commas.
    -f web api prod           Space separated. (Avoid). Ambiguous with positionals.

    Key=value pairs
    --label env=prod --label region=eu-west-2       Flag can be repeated. One new pair from each.

    Enum / choice
    --format json|yaml|text         List the valid values in help and error.

    '-' stdin / stdout
    cat data.json | zed -a -        '-' universally means stdin / stdout.

    Reserved convention options
      -h / --help        help                                               
      -V / --version     version                                            
      -v / --verbose     verbosity (note the -v/-V collision — pick a side) 
      -q / --quiet       suppress non-essential output                      
      -o / --output      output destination                                 
      -f / --force       skip safety checks                                 
      -n / --dry-run     show what would happen                             
      -r / --recursive   recurse                                            
      -y / --yes         assume yes to prompts             

3. The Config Cascade

    Precedence:
    built-in defaults  <  config file  <  environment variables  <  command-line flags

    Env vars: 
    screaming snake, prefixed with the app name - MYAPP_LOG_LEVEL

    Secrets:
    secrets belong in env vars or files, never in a flag, because flags are visible via ps.

4. Misc

    Exit codes:
        0 - success
        2 - usage error
        other - failure

    TTY detection:
        colour, progress bar, prompt -> only when stdout is a terminal

    Machine mode:
        --json or --format json     for machine readable output

*/

use clap::{Parser, ValueEnum};


// _____________________________________________________________________________________________
// 0. Add clap library:  
//    cargo add clap --features derive



// 1. Define a struct - its fields become positional arguments; and optional flags; doc comments above struct fields become help text; annotations add things like cli name, description, version. 
//    Clap reads the struct and generates: the parser, validation, --help, error messages, etc.
#[derive(Parser, Debug)]
#[command(name = "greet", version = "0.1.0", about = "Says hello")]       //  "name" and "version" should match the binary name and version
struct FlatCli {

    // Positional arguments ----------------------------------------
    /// Who is doing the greeting   <- Doc comment becomes the help text
    greeting: String,

    /// Who is being greeted
    dst: String,

    // Optional flags ----------------------------------------------
    // long         generate long version
    // short        generate short version
    //
    // 7 ways to make a flag optional (absence must be representable somehow):
    //   1. Option<T>                       -> absent = None, no CLI-level default
    //   2. T + default_value = "literal"   -> absent = literal, parsed via FromStr
    //   3. T + default_value_t = expr      -> absent = expr (needs Display; enums use ValueEnum instead)
    //   4. bool                            -> presence/absence flag, defaults false (ArgAction::SetTrue)
    //   5. Vec<T>                          -> zero occurrences = empty vec, no Option needed
    //   6. env = "VAR" (+ default_value)   -> falls back to env var, then to default_value
    //   7. num_args(0..=1) + default_missing_value -> flag optional AND bare flag (no value) optional

    /// 2. default_value: string parsed via FromStr when absent
    #[arg(long, default_value = "Hello")]
    prefix: String,

    /// 1. Option<T>: absent -> None
    #[arg(long)]
    output: Option<String>,

    /// 3. default_value_t on an enum: needs value_enum + ValueEnum (which requires Clone)
    #[arg(long, value_enum, default_value_t = GreetingStyle::Normal)]
    greeting_style: GreetingStyle,

    /// 1. Option<T> applied to an enum: absent -> None
    #[arg(long, value_enum)]
    style: Option<GreetingStyle>,

    /// 4. bool: presence toggles true, absence = false
    #[arg(long)]
    verbose: bool,

    /// 5. Vec<T>: repeatable flag, zero occurrences -> empty vec
    #[arg(short = 'v', long = "volume")]
    volumes: Vec<String>,

    /// 6. env fallback: flag > env var > default_value
    #[arg(long, env = "GREET_LANG", default_value = "en")]
    lang: String,

}

#[derive(Clone, Debug, ValueEnum)]
enum GreetingStyle {
    Normal,
    Enthusiastic,
    Casual,
}

fn main() {
    let cli = FlatCli::parse();
    println!("{cli:#?}");
}


