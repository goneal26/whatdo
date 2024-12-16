# whatdo

The CLI for when you can't decide what to do next.

I've got a lot of different projects that I like to work on, and a lot of 
different things I need to get done. These often all end up having the same 
priority, so I made this program to just pick one for me to work on instead.

`whatdo` is a to-do list that adds your tasks to a "queue" that works a bit
like a music playlist- you get a different ordering of every task in the
"queue," and after going through each item the list is re-shuffled to a new 
order.

## Features

- Add and remove notes for things you want to work on to a "queue".
- Pick a random thing from the "queue" for you to do next.
- Reshuffle the "queue" if you don't like how it's ordered.
- Queue is stored in a human-readable TOML file

## Usage

Grab a random unfinished task to work on from the queue:

```
whatdo
```

You can also use `whatdo pick` to achieve the same result.

Add tasks to the queue directly as single-word arguments:

```
whatdo add painting reading biking
```

Tasks are single-word items separated by spaces.

List all tasks:

```
whatdo list
```

This will return the entire list, with each task on a separate line.

Remove a task by name:

```
whatdo drop reading
```

Remove all tasks:

```
whatdo clear
```

Reshuffle the queue (if you don't like the ordering). This is done automatically
after you've ran `pick` for every item in the queue:

```
whatdo shuffle
```

See help and other information:

```
whatdo help
```

You can also use `whatdo --help` or `whatdo -h`.

See version info:

```
whatdo --version
```

You can also use `whatdo -V` instead.

## TOML list storage

Your "global" task list is stored in a TOML file, alongside the queue containing
the next hobbies the program will "pick" (in case you want to spoil the 
surprise).

Here is an example:

```toml
list = [
    "reading",
    "swimming",
    "biking",
    "painting",
]
queue = [
    "swimming",
    "biking",
]
```

To find the path to this file, enter:

```
whatdo path
```

By default, the task list is named `list.toml` and is stored in your local 
config directory. So, on linux, that's something like `~/.config/whatdo/list.toml`.
