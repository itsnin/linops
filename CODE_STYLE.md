# code style rules

## comments
- all comments must be lowercase
- no punctuation in comments no periods no exclamation marks no commas
- explain why but if this specific line is really an obscure thing that many dont even know then explain what as well but only for obscure stuff dont trash talk in comments
- use // or # or something else depending on what language it is
- no block comment boxes no jsdoc blocks no docstrings no banners
- no references to other projects libraries or extensions in comments but if this specific thing is not common or obscure that many dont know then provide evidence with verified working links for with why and what so people would actually understand it so theyll know its right
- no llm smell phrases like "here we" "lets" "we need to" "note that" "important" "todo" "fixme"
- comments should look like a human devs quick marginal note not documentation but ofc you gotta do different for obscure codes or something that needs verification to be trusted

## code structure
- split logic into many small files each with a single responsibility
- keep enable/start and disable/stop or equivalent lifecycle methods next to each other for easy review
- one concept per file one file per concept

## anti ai code smells
- do not wrap standard api calls in try catch blocks
- do not use try catch to silence errors that should never happen if a function can fail return null or use an explicit error type
- do not use optional chaining or nullish coalescing for methods that are guaranteed to exist
- do not add defensive null checks that mask bugs instead of handling them
- do not add just in case code for situations that cannot occur

## review discipline
- before producing final output read every single line you wrote
- look for potential issues on every files not just the file you are currently editing
- when fixing a bug check whether the same bug pattern exists elsewhere in the codebase
- do not assume a fix works verify it against the actual code
