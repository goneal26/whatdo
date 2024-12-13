# whatdo

The CLI for when you can't decide what to do next.

This is a simple command-line program for picking between a random "hobby" from 
a list of hobbies. I've got a lot of different projects that I like to work on,
so I let this program pick one for me.

## Features

- Adds items to a list of "global" tasks.
- Randomly select a task from that list to work on.
- Global task list stored in a human-readable text file.

## Usage

Grab a random unfinished task to work on from the global list:

```
whatdo
```

whatdo essentially "shuffles" the tasks when you do this. This way, the 
same task is not picked twice in a row, and it's not until it has gone through
every task that you see the same one again.

Add tasks to the list directly as single-word arguments:

```
whatdo add painting reading biking
```

List all tasks:

```
whatdo list
```

Remove a task:

```
whatdo drop reading
```

Remove all tasks:

```
whatdo clear
```

## Configuration

Your "global" task list is stored in a text file, with each task being written
on a separate line. Here is an example:

```
painting
reading
biking
swimming
```

To find the path to your current config, enter:

```
whatdo path
```

By default, the task list is named `todo.txt` and is stored in the same 
directory as the program's executable.

The location of the config file used by the program can be changed:

```
whatdo set-path /path/to/todo.txt
```
