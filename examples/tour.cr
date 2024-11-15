# assignment
n = 10
greeting = "hello world"
truth = true

# text
message = "the greeting is {greeting}"
write(message)

# scopes
x = 5
do
    x = 10   # shadows the previous value of 'x'
    write(x) # prints 10
end
write(x)     # prints 5

# variables
var y = 1 # y is now variable
do
    y = 2
end
write(y) # prints 2

# control flow
is_valid = true
answer = 5
if is_valid and answer < 10 then 
    write("it was true")
else
    write("it was false")
end

# pattern matching
line = "hello"
reply = match line with
    case "hello" then "greetings"
    case "goodbye" then "farewell"
    case _ then "huh?"
end
write(reply)

# loops
var sum = 1
loop
    if sum < 10 then 
        break 
    else 
        sum = sum + sum
    end
end

# types
n: number = 42.0        # types are typically inferred from usage
str: text = "text_here" # but can be specified

# functions
function add(x: number, y: number): number
    # implicit return
    x + y
end

# lists
list = [1, 2, 3]
second_element = list.2

texts = ["are", "you", "having", "a", "good", "day?"]
write(texts.join(" "))

sum = list.sum()
write(sum) # prints 6

# tuples
result = (42, "life", true)
(n, t, b) = result # destructuring

# types
type id is number end
type expr is
    atom(number),
    op(text, expr, expr)
end

function eval(expr: expr): number 
    match expr with
        case expr.atom(n) then n 
        case expr.op(op, a, b) then match op with 
            case "+" then eval(a) + eval(b)
            case "-" then eval(a) - eval(b)
            case "*" then eval(a) * eval(b)
            case "/" then eval(a) / eval(b)
        end
    end
end

# records
type id is number end
record message is
    id: id,
    message: text,
    status_code: number,
    is_valid: bool
end

function message.check(self: message)
    if not self.is_valid then
        write("message is invalid")
    else end
    match self.status_code with
        200 then write("OK: {self.message}")
        other then write("unhandled status code: {other}")
    end
end

# traits
#record book is
#    title: text,
#    description: text
#end
#record article is
#    title: text,
#    byline: text
#end

#trait summary is
#    function name(self): text
#    function description(self): text
#end

#record book has summary
#    function name(self): text self.title end
#    function description(self): text self.description end
#end
#record article has summary
#    function name(self): text self.title end
#    function description(self): text self.byline end
#end

#function summarize(thing: summary) 
#    name = thing.name()
#    description = thing.description()
#    write("{name}: {description}")
#end

#summarize(book(title = "Crafting Interpreters", description = "A book about making interpreters."))
#summarize(article(title = "Cool Programming", abstract = "10 features you won't believe exist."))

# more literals
decimal     = 1_000.1
hex         = 0xffab
octal       = 0o77
binary      = 0b1010_0101
scientific  = 1.2e3

# format string
format   = "number {n}, predicate {p}" 

# special characters
special  = "this text contains a tab: '{core.text.tab}' and a newline: {core.text.break}See?" 

# regexes
iso_8601 = core.text.regex("(\d{4})-(\d{2})-(\d{2})")

#c_string = "some data string"          # contains null byte at the end

# ranges
range = 1..5          # [1, 2, 3, 4, 5]
range_from = 1..     # [1, 2, 3, ...]
