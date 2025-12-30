# nous - a note-taking system

A note-taking system where

- a note is an object with a body, aliases, tags, attributes, and a immutable 
  identifier, and
- everything is a note.

Notice that notes do not have dedicated titles; they instead have aliases. The
goal is to reduce the friction of capture, as one would not need to come up with
a title for a note then and there. Instead, one can refer to notes by their 

- ID, which is automatically assigned, or
- alias, which is optional -- one can assign them post-hoc, if at all.

The body itself is a list of blocks, where a block is either a Markdown body or
a note ID.

- Note IDs that are used as blocks are transcluded rather than linked; referring
  to the parent note will also refer to the children, though the inverse may not
  hold.
- Markdown variants may not be adjacent; adjacent Markdown blocks are merged.

## The conveyor-belt workflow

Nous aims to assist in separating the note-taking workflow into stages, and
provide the user with precisely the context necessary for every particular
stage. While the exact structure of these stages may vary for each user, in
general, one may categorise them into

- _capture_, where the goal is to quickly but comprehensively input information 
  without worrying about structure, and
- _review_, where the goal is to contextualise the captured information, link
  them to others, split them up into many notes or combine many into one, assign
  tags and aliases, _etc._.

By worrying about structure on the review stage, we can focus on completeness on
the capture stage, and by worrying about completeness on the capture stage, we
can focus on completeness on the capture stage.

## Everything is a note

A note is simply a body of text with an identifier and other associated 
attributes. These notes can be further specialised by assigning _types_ to them.
Nous provides some built-in types, namely

- _tasks_, which are notes with a due date, priority, dependencies, and
  state (to-do, blocked, in progress, done - the user may define their own),
- _views_, which we will discuss later,
- _events_, which are notes with a start date and end date, and
- _operation logs_, which are special notes that keep track of changes to other 
  notes, except other operation logs themselves.

Additionally, users may define custom types to fit their own use cases. The goal
is to widen the range of information that can the user may capture and assigned 
structure to.

Currently in consideration is the use of [traits] as a first-class concept. This
would in theory allow for, say, the distinction note types with actionable 
dates, such as tasks and events, versus notes with non-actionable dates (every
note has a time of creation and time of last modification, but these are not
actionable in the same sense as the due dates of tasks or the start/end times
of events). This may be useful to enrich queries.

[traits]: https://en.wikipedia.org/wiki/Trait_(computer_programming)

## Filtering out the noise

The common principle of note-taking methodologies such as Zettelkasten is to
be judicious with the information included in one's notes to maintain a high 
enough signal-to-noise ratio to be useful. While this is a prudent decision when
accounting for the inherent restrictions of physical notes, this may not be as
crucial for digital notes, where querying one's notes is vastly more trivial.

Consider the impossibility of determining,  beyond the scope of estimation, what 
information will be useful in the future. By excluding or deleting some piece of
information, the worst-case scenario is that said information is no longer 
recoverable in the future when one needs it. By instead filtering for relevant 
information, one may similarly restrict their working context while avoiding the
permanent loss of information.

Therefore, instead of being selective with the information captured, Nous aims 
to provide flexible and precise querying capabilities instead. The idea is:
capture everything, and filter for what you need.

Nous provides two constructs to achieve this, namely

- _queries_, which are one-off, ad-hoc search operations, and
- _views_, which are a type of note which encodes queries, and thus is more
  permanent in nature.

Views could even be referred to in notes, _e.g._

```
TODO Write up a grocery list. Look at things we may run out of soon [[view::
(tag:observation AND tag:grocery AND ctime<=1w)]]
```

Like other notes, one can assign aliases to views, and can subsequently refer to
them by their aliases:

```
TODO Write up a grocery list. Look at things we may run out of soon 
[[view::grocery-observations]]
```

Views have the `scope` attribute, which may refer to other views for the scope
of its search. If a view is defined as its `scope`, then it will filter through
the results of that view. Otherwise, it will filter through the global set of
notes.

### Searching

Searching has an equivalent syntax to views, but done one-off in the command
line:

```bash
nous search tag:observation AND tag:grocery
```


### Structural queries

The Markdown body of notes can be queried in a structural manner as well:

```bash
nous search --id tag:error-handling AND (NOT tag:grocery) | \
  nous search heading --level --level 1-3 --label "*Example$" --scope stdin | \
    nous search codeblock --lang rust --scope stdin
```

The first query finds any notes that has the tag `#error-handling` and not
`#grocery`, and returns the IDs. They are piped to the second query, which
searches through the notes returned by the first query and finds any level 1-3
headings with titles ending in "Example". Lastly, the third query takes the IDs
returned by the second query and finds Rust code blocks within them.

### Date and time

Notice that the above example encodes dates in relative terms. In addition to 
the ability to specify dates in absolute terms, _e.g._ `2025-12-30`, one could
also override the "now" anchor of relative dates in views by using the `@`
notation. For example,

```
TODO Write up a grocery list. Look at things we may run out of soon 
[[view::grocery-observations@2025-12-30]]
```

or

```
TODO Write up a grocery list. Look at things we may run out of soon [[view::
(tag:observation AND tag:grocery AND ctime<=1w@2025-12-30 and mtime<=5d)@2025-12
-30]]
```

overrides the anchor for the entire view, while

```
TODO Write up a grocery list. Look at things we may run out of soon [[view::
(tag:observation AND tag:grocery AND ctime<=1w@2025-12-30 and mtime<=5d)]]
```

overrides it only for the `ctime` clause.

### Graph traversal

Nous also exposes graph traversal primitives:

```bash
nous traverse [--strategy bfs,dfs] [START ID]
nous edges [--from ID] [--to ID]
```

which returns the traversed nodes and edges as JSON, allowing composition with
external scripts and tools, _e.g._ to implement global PageRank or Personalised
PageRank.

## Identifiers

The immutable identifier of each note is constructed in a similar style to
[Jujutsu's] change ID: random alphabetical strings that can be referred to by
some short prefix.

Individual syntax elements are also addressable by the same ID scheme, and are
thus searchable and transcludeable in other notes.

[Jujutsu]: https://github.com/jj-vcs/jj

## Semantics

The default view should be some sort of timeline of notes. By default, it 
restricts the default view to notes created today (or tasks due today, events
scheduled for today, _etc._), but this can be configured.

There are to competing paradigms of a timeline that I am considering currently,
which are

- an outline, which means that each note has an optional parent and child; this
  seems to be the most straightforward option, as it is intuitive for most
  people to write outlines, and it is already tried with tools such as Roam
  Research, or
- a "commit/change history" model similar to the Jujutsu VCS, though with the
  difference that there may be more than one starting point to the history. I'm
  not sure how useful it is to have multiple parents or children when links
  exist, though. The parent/children relationship is, to begin with, just a
  syntactic sugar to make the flow of thought easier; I'm not convinced it's
  actually useful as an ontological construct otherwise.

Currently, I'm more partial to the outline model. Running the CLI with no
argument, _e.g._

- `nous`, should show today's timeline, with notes sorted by created time, tasks
  sorted by due time, and events sorted by start time, 
- `nous new <TEXT>` creates a new, plain note,
- `nous new <TEXT> --parent <ID/ALIAS>` specifies a parent,
- `nous new <TEXT> --type <TYPE> --attr1 <TEXT> --attr2 <TEXT> ...` specifies the
  type and associated attributes, _etc._
- `nous edit <ID/ALIAS> <TEXT> --parent <PARENT> --type <TYPE> --attr1 <TEXT> ...`
  edits the note.

There's no reason the default `nous` view should be a timeline of "today", and
it should be configurable; it's simply what Roam does, and I think it's nice to
have a clean slate every day.

### Showing

Running `nous show <ID/ALIAS>` should show a timeline with the root set at that
note, followed by a sorted timeline of other notes referencing it. This may be
augmented by the note type; for example, for a task, the timeline may also
include in addition the history of state change, logbooks, _etc._ A good 
reference for what the per-note view should look like would be GitHub issues,
only with one user rather than multiple. The idea is to make it easy to "retrace"
your steps. For example,

```
○ rvrounum | TODO | Tomorrow | Every week
│ Buy eggs
○ tqskrylv | Operation | Yesterday 
│ Recurrence interval of task `rvrounum` set to "Every week"
○ wmkxknkx | Note | 6 days ago | #observation #groceries
│ We ran out of eggs [[Buy eggs]]
○ ownvvymo | Note | 13 days ago | #observation #groceries
│ We ran out of eggs [[Buy eggs]]
```

Given this sort of view, it is trivial to deduce that we set the recurrence
interval of the task "Buy eggs" to "Every week" because of past observations 
that we seem to always run out of eggs every week.
