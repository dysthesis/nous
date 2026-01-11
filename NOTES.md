Design
======

Requirements
------------

- Capture is as quick and frictionless as possible
- Notes can be annotated as richly as possible later
- Search can be as quick and simple, or as complex and precise as you want
- Bidirectional linking
- Is as flexible as the mind; can be used to capture and model _everything_.
- Should act as a "cache" for the brain.
  - The de-facto cache replacement policy is least-recently used
    - TODO: Define what "used" means; is it enough to have viewed some note, or
      should it be necessary to edit it as well?
- There's this notion of bankruptcy that is commonly found when it comes to
  implementing systems such as Emacs or Neovim configuration; that is, when the
  system not only is complex but is tangled -- too much complexity lies beyond
  the bounds of the existing organisational system -- that it is more efficient
  to simply start over than fix it. Since starting over is lossy, that is, you
  can potentially lose information by doing so, the system should be as 
  resistant to bankruptcy as possible.
  - TODO: define bankruptcy in this context
- Leverage automation as much as possible to minimise friction.
  - Automation can do many things, but it cannot capture or define intent. Use
    it for everything else that does not require that.
  - Automation shifts the burden and failure mode of manual maintenance to the
    implementation of the automation mechanism, which may be more deterministic
    but less flexible than the human.
- Strong separation of mechanism and policy in order to promote the system's
  longevity. Policy is temporary, and changes with you and your life, but the
  underlying mechanism is generally agnostic and reusable; it would be a waste
  to throw away the mechanism with the policy.
- The system should be gracefully degradable: if I lose access to the tool I'm
  building, I should still be access the complete information stored perfectly
  losslessly. I should only lose functionality, not information.
- Must be as fast. This is to prevent the brain from context switching to 
  something else subconsciously, and maintain focus.
  - Look into what the exact scientifically proven time is for the brain to lose
    focus while waiting.

Research
--------

- We inform the design of our system based on the principle that information, on
  a human level, lies on a spectrum with two axes, namely urgency and 
  significance. We can divide the spectrum into quadrants to form cases, and 
  identify the constraints of each, in order to turn them into an optimisation
  problem.
  - High urgency implies that the constraint is time; the only way to reliably
    reduce the time necessary to input is to reduce how much information we need
    to capture. Therefore, frictionless capture is necessary -- we identify 
    which information must be captured now, and which can be delegated to later.
    - For example, if a lecturer announces that you have an assignment due next
      Friday, you have two pieces of information that you must capture _now_,
      which is that you have an assignment to do, and that it's due next Friday.
      Everything else, such as what course the assignment is for, what the 
      marking rubric is for it, how to decompose it into actionable child tasks,
      can be done later, and thus should be done later.
  - TODO: identify if there are other variables that could be used to 
    comprehensively and exhaustively model the problem of desigining this system
    into sub-problems that are just optimisation problems.
    - Reacquisition cost could be a further axis: how difficult is it to 
      reacquire non-urgent information later?
      - Technically, reacquisition cost might be infinite -- that is, it is 
        impossible to reacquire it later. But I think it might be more sensible
        to map information that is impossible to reacquire later as 
        high-urgency?
    - Error-sensitivity could be another axis. Some ideas can be paraphrased 
      without consequence, others can be fatal (e.g. you don't want to 
      paraphrase legal terms, numbers, exact quotes, etc.).
    - Attention, but this is applies also (or perhaps more accurately, even 
      moreso) to retrieval on top of just capture.
    - Further TODO: could there be arguments against the methodology of 
      modelling the larger problem of system design as smaller sub-problems of
      optimisation?
      - The general heuristic of counterarguments against optimisation is that
        it might be myopic (see: greedy problems). Does it apply here?
- Zettelkasten is very good for what it's used for: research.
  - It's a very good methodology for keeping track of ideas
  - But not everything is ideas
    - E.g. tasks, events, transactions, bookmarks, etc. are not ideas
- *Question.* Should all these information coexist in the same system?
  - Arguments for
    - Leverage emergent properties. _E.g._ the existence of tasks can be explained
      by some idea notes, if you have notes on finance and economics, you can
      link them to notes of your own real-life transaction records in order to
      plan out how to optimise your finances better.
  - Arguments against
    - Increases clutter. While all the extra information is not strictly noise,
      it can be in different context. I don't want to see records of what I
      got for groceries yesterday while I'm writing notes on fully homomorphic
      encryption.
        - This is solvable by strong queries and filter systems.
            - This requires both a flexible annotation system (tagging, etc.) in
              order to associate auxiliary information necessary to run queries
              through it, as well as a sufficiently disciplined practice of 
              doing so.
- There exists a Goldilocks' zone of complexity
  - a system that is too simple is prone to disarray, mixing of semantically 
    disparate signals (e.g. using tags for everything from todo states, note 
    types, details, etc. when they inherently mean different things), and limits 
    the growth and malleability of the system,
  - but one which is too complex risks mismanagement -- maintenance of the 
    system relies on user discipline, and the more complex the system, the more 
    tedious the maintenance and the higher the risk of the user not being
    disciplined enough to do so. 
  - Automation helps alleviate the burden of complexity, but not everything can 
    be automated (_e.g._, the computer cannot automatically assign semantic
    information -- that is, those related to intent -- to your notes)
    - Technically they can, via LLM, but
      - they are not accurate, and
      - this just shifts the burden from annotation to recall; you now have to
        think, how would the LLM (who is also not a principled human whose 
        behaviour can be inferred, but a stochastic machine!) label and annotate
        the information that I am looking for?
- Folders are really not great; they are overly restrictive by being mutually
  exclusive.
  - Maybe there exists a use case for such a property that I have yet to come
    across, but I don't know.
  - Filters/queries/transforms may be a better first-class construct than 
    folders
    - They are not mutually exclusive
    - They are inherently automated
    - Transforms can not just find information, but rearrange/reorganise them in
      different ways, e.g. to make a dashboard
      - Transforms should be bidirectional: you should not just be able to, say,
        automatically create a bullet list outlining all the notes you have, but
        also have any changes to that generated list reflected back to the 
        system
        - E.g., if you add or delete items, those should correspond to the 
          addition or deletion of those notes. If the original query was for 
          notes of specific tags or types, then the newly created notes should
          also be automatically assigned those tags or types.
    - Here, views (transforms/queries/filters) are just categories? I don't 
      think it cleanly maps to a category, but it's the closest mathematical 
      construct that I can think of.
      - See if there are better constructs that has a stricter bidirectional
        mapping to this.
      - Otherwise, see if you can use category theory to inform the construction
        of views.
    - The responsibility of these filters/queries/transforms (maybe call them
      views or lenses or something?) is to define and enforce context, such that
      we can capture everything into this system, but only information that is
      relevant to the action at hand is ever displayed or easily accessible.
      - **Question.** Is the automatic detection of such context possible, such
        that the user is not restricted to manually switching between these 
        views? I don't think this is that sensitive to errors, so it may be 
        possible to do so using some machine learning models?
- What about information density? How dense do we want it to be, and how do we
  enforce it in the system? If there's a universal correct answer, we make that
  a mechanism; otherwise, it is a policy.
  - Look into the use of information theory to inform the construction of the
    system.
  - Information theory tells us that the denser the store of information is, the
    more efficient it is to transport (including transporting back from the 
    system to our brains), but the less redundant it is; the consequence of the
    latter being that it is more sensitive to errors and deviations. Where in
    this efficiency vs. resilience trade-off do we want to hedge our bets?
- The system should be designed with an LLM as a model of the human mind
  - Like LLMs, humans are a "roll of the dice" -- the brain is made to have 
    ideas, not store or manage them.
  - The trend in agentic language models is that a "dumber" but more mechanical,
    deterministic system is necessary to enforce structure to the chaos. With
    LLMs, those are the CLI wrappers around it, bundling function calls, context
    management, etc. For humans, it's our note-taking system
    - Of course, this begs the question: what's a good structure to enforce? I
      think that is more policy than mechanism, though.
  - Therefore, like LLMs, our system should be there to manage things that we
    don't have the discipline to do, e.g. storing and recalling information.
    - How information is recalled forms the foundation of the structure
      enforcement mechanism.
  - One limitation of this model is that LLMs don't feel tiredness or laziness;
    humans do. However, LLMs experience a similar eventual degradation in the 
    form of context decay. Different reasons, same consequence.
- TODO: enumerate the modes of failure comprehensively for this system, and
  decide which one we care about (and how much).
- Consider making notes immutable; the id becomes just the hash of the entire
  note itself, and a base note type would therefore have a predecessor and
  a possibly empty list of successors. to prevent size explosion, periodic 
  pruning can be done.
- Consider making a note immutable, and therefore 
  - editing a note just means creating a new note with the old version as its
    predecessor, and
  - the ID of a note is a blockchain-style hash that records both its content
    as well as who the predecessor is.
  - See: [Content-addressable storage]
- Consider [Messaging-app-like UI]
  - In a terminal, maybe this would be closer to a REPL
  - Beyond UI, you can even map note-taking as texting (discourse)
    - This is supported by principles such as Zettelkasten, which posits that
      taking notes is just discourse with yourself.
    - The whole automation aspect is therefore similar to moderation in 
      discourse platforms such as forums or groupchat: you automatically 
      moderate discourse with yourself.
    - Is this policy as opposed to mechanism, though? If so, perhaps a separate
      UI layer should be implemented over the core tool to implement this.
- TODO: Examine prior art in notetaking tools (e.g. Obsidian, Org-mode, Roam)
  and see how they compare against our requirements.
- TODO: Category theory tells us a lot about how objects compose. Look into how
  category theory can be used to inform the construction of mechanisms to 
  compose atomic notes.
- Latency is a key consideration in order to maintain user focus
  - The goal is <100ms latency for any operation; that is the threshold at which
    operation is perceived to be instantaneous to humans.
  - The hard limit is 1s for any operation; that is the threshold at which 
    attention will start to wander.
  - Time complexity is a key consideration, but only to a certain extent.
    Asymptotic analysis studies how the runtime grows as the input size 
    approaches infinity, but realistic workload does not involve asymptotic
    input sizes. Consider if there exists algorithms that may run better at our
    expected input sizes, even if it may be slower asymptotically.
    - TODO: What is our expected input sizes? This is generally the expected
      size of `contents/`.
      - TODO: What are the "categories" of inputs we may have? Of course, the
        worst case is always the whole repository, but some operations may
        provably only ever use some specified subset of the repository. See if
        it is worth considering these special cases on top of the worst case.
  - Space complexity is important, but only in the aspect that it improves the
    algorithm's locality of reference.
    - See: [Cache-oblivious algorithms]
    - For example, [funnelsort] may be
      more optimal for when the array to sort is larger than the cache
      - For reference, the size of an L1 cache in `znver4` is 64 kB per core. If
        we're sorting the entire repository, we're almost guaranteed to pass 
        that threshold.
      - Repositories may be small; it might be worth implementing a switch such
        that different algorithms are used for different special cases, e.g.
        merge sort for 64 kB input sizes, funnelsort for anything beyond
      - We could define variables per-platform using build-time conditionals
        (_e.g._ `IFDEF`, `#[cfg_attr]`, etc.) to inform the program about cache
        sizes.
  - Benchmarking is important! Learn about designing benchmarks in a 
    statistically sound manner.
    - See [Gernot's List of System Benchmarking Crimes]
      - Also this [related paper]
      - Every type of workload should be benchmarked. Any omissions should be
        justified.
      - The range of inputs (independent variable) should be justified to prove
        that it demonstrates a representative workload.
      - Any claims of performance improvements must demonstrate
        - the *progressive criterion,* which shows that performance does improve
          in the areas of interest, and
        - the *conservative criterion,* which shows that performance did not
          meaningfully regress everywhere else.
      - Standard deviation must be considered to justify the significance of the
        result (that is, the result is not a flux!)
- We expose two primary means of interfacing on the terminal layer with two 
  different goals: scripting, and interactivity. These two should have the 
  exact same surface area, just in different forms.
  - For scripting, we obviously use a CLI interface
  - **Question.** For interactivity, do we go with REPL or TUI?
    - TUI is pretty and flexible, but can be bloated
    - REPL is more minimal, lean, focused, but also, how do we implement 
      incremental, interactive search?
      - To a certain extent, we can implement a fuzzy completion menu similar to
        what Codex or Claude Code CLI has, but I think that has its limitations;
        I don't know how that would work with a DSL.
- TODO: Look into a querying DSL.
  - It should be composable, such that complex queries should be expressible as
    a combination of simple ones
  - The complexity of the syntax should grow with the complexity of what it's
    trying to express. Simple queries should be expressible with minimal syntax.
    - To borrow concpets from information theory, we want maximum entropy on the
      querying syntax.
    - Maybe we could borrow ideas from Morse code: the more frequent a query is
      used, the simpler/shorter the DSL encoding it should be.
      - How do we formally determine this though, without just actually 
        gathering data and running statistical analysis, which is inherently
        probabilistic?
  - Personal preference: I want the syntax to look more like Haskell and less 
    like Lisp, but this is compromisable (Lisp tends to be the most expressive)
    - See: tree-sitter syntax for querying and transformation
- TODO: Look into whether incorporating the syntax tree in this tool is useful, 
  and whether it violates the single-responsibility and minimality principle.
  - Arguments for
    - We need to parse the document to find links and render anyways, so might
      as well make the generated syntax tree queriable.
  - Arguments against
    - We could better adhere to minimality and SRP by separating the structural
      search into a separate tool; the store of data is a flat directory of
      Markdown files anyways, it should be trivially interoperable.
    - If all we're parsing are links and wikilinks, they may be better parsed
      by using regex (and maybe they'd be more amenable to SIMD optimisations?)
      - But we might not just be parsing links since we need to render
        - Then we could use pulldown-cmark to lazily parse on render; the 
          compute necessary to parse a string in memory may be faster than 
          I/O fetching a cache from disk, anyways.
- Maybe views shouldn't be a first-class construct if query is already a thing,
  as per the minimality principle?
  - Arguments for
    - Having it as an internal construct allows us to cache views
      - We can cache query results as well anyways, though maybe having views as
        a semantic distinction allows us to just pin the cache and not have it
        purged
  - Arguments against
    - It has no difference with a regular query other than that the tool 
      remembers it for you; it is effectively a _named_ query; you can implement
      something like that externally, and therefore should.

Storage
-------

- To map to our requirement of quick capture, flexibility, and longevity, we
  use a flat directory of content-addressable Markdown files as the source of
  truth.
- An SQLite database is used to cache the contents and search faster, but with a
  strict requirement that no information is stored in the database that is not
  stored in the plaintext directory.

```
├──  config.toml
├──  index.db
├──  attachments
├──  contents
│   └──  [IDENTIFIER].md
├──  templates
├──  types
└──  views
```

[Content-addressable storage]: https://en.wikipedia.org/wiki/Content-addressable_storage
[Messaging-app-like UI]: https://strflow.app/
[Cache-oblivious algorithms]: https://en.wikipedia.org/wiki/Cache-oblivious_algorithm
[funnelsort]: https://en.wikipedia.org/wiki/Funnelsort
[Gernot's List of System Benchmarking Crimes]: https://gernot-heiser.org/benchmarking-crimes.html
[related paper]: https://arxiv.org/pdf/1801.02381
