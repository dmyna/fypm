## TYPE
##### Habit
A task with `Habit` has a recurrence setting and must have `STATE` set to `Time`. This type of task means that it is a habit, something that you must do every day (either automatically or consciously) and that can be counted.
##### Objective
An `Objective` task is basically a goal. This type of task is ideal for deadlines longer than 1 day and is generally not completed in less than that same time. This type is also the only one that can store a `MOTHER` attribute.
##### Eventual
A task with `Eventual` type means that it will be done on the same day that it was entered. That type of task is reserved to tasks you didn't know you needed to do. Useful to unplanned meetings, quick tasks, etc.
##### Continuous
A task that has its `TYPE` set to `Continuous` will never be completed. Continuous tasks need to be... Continuous. This type of task serves as a counter, a good use case is to create a task like "Minecraft" to know how long you have played in total.
- You can also use `taconls` to see all continuous tasks.
##### Event
That type of task is useful to schedule commemorative dates or automatic payments. A task with `Event` type must accompany a `WT` set to `AllDay`, otherwise it can simply be a `Check` task. This type of task need a day to be completed, and the time that will be setted is always "23:59:59" in the day that was defined.
##### SubTask
##### Check
Unlike the `Habit` type, this type does not come with a `STATE` defined as `Time`, but rather defined as `Info`. This has the same logic as Habit, but it is not possible to start this type of task. Useful for statistical tasks like "Sleep on time", "2000 calories", etc.

## STATE
##### Time
Tasks with `Time` `STATE` can be started and logged in Timewarrior. This is the normal and default `STATE`.
##### Info (and INFORELAT)
Unlike `Time`, the `Info` `STATE` can't be started and it won't appear in Timewarrior logs. This is for informative tasks that are worth more as a "call", and not as a task in itself. Tasks with `MOTHER` set will always have the `STATE` attribute set to `Info`, because you cannot start them.
But... If you try to start one of these tasks? There are two possible cases:
- The first, obviously, is an error. The Fypm will not let you start a task like that and nothing will happen after that.
- Now if you have the `INFORELAT` attribute set to a valid uuid, the program will redirect to this task.  

## MOTHER
The mother attribute stores a valid uuid that creates a connection between the SubTask and the mother task (it will always be an `Objective` task).

*But... What "connection" means in this context?*

Simple, not only will you know which task is the main one, but you will also have algorithms like `taadd-seq`, `tapas` and `tastart` that use this connection and will give you facilities precisely because of that.