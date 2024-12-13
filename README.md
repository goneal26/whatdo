# hobbypicker (working title)

For when you've got too many hobbies.

This is a simple command-line program for picking between a random "hobby" from 
a list of hobbies. I've got a lot of different projects that I like to work on,
so I let this program pick one for me.

## Features

- Adds items to a list of "global" tasks.
- Randomly select a task from that list to work on.
- Mark the tasks as complete/incomplete.
- Global task list stored in a readable Markdown file.

## Usage

Grab a random unfinished task to work on from the global list:

```
hobbypicker
```

Hobbypicker essentially "shuffles" the tasks when you do this. This way, the 
same task is not picked twice in a row, and it's not until it has gone through
every task that you see the same one again.

Add tasks to the list directly as single-word arguments:

```
hobbypicker --add painting reading biking
```

Mark a task as done:

```
hobbypicker check taskname
```

Unmark a task as done:

```
hobbypicker uncheck taskname
```

Clear all completed tasks from the list:

```
hobbypicker clean
```

List all tasks:

```
hobbypicker list
```

## 

## Configuration

Your "global" task list is stored in a config file. By default it is placed in 
the same directory as this program. This file is a Markdown file containing a 
task list, like so:

```
- [ ] task1
- [x] task2
- [ ] task3
- [ ] task4
```

With the `x` marking a completed task, and a space marking an incomplete task.

To find the path to your current config, enter:

```
hobbypicker --config
```

By default, the task list is named `todo.md`.

The location of the config file used by the program can be changed:

```
hobbypicker --set-config /path/to/config.md
```
