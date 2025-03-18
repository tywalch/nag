# nag

A cli reminder for people who need a little more than just a ding and toaster message

## Installation

To install **nag**, follow these steps:

1. Open a terminal.

2. Navigate to the directory where the `./install.sh` script is located:

Run the following command to install **nag** to the default directory `/usr/local/bin`:

> Note: you may need execute the script with the appropriate permissions 

```sh
./install.sh
```

This will make the **nag** globally.

Alternatively, if you want to install **nag** to a custom directory, specify the desired path as an argument to the install.sh script. For example, to install **nag** to `/path/to/custom/directory`, run:

```sh
./install.sh /path/to/custom/directory
```

Wait for the installation process to complete.

After following these steps, _*nag*_ will be installed and ready to use! Assuming the installation path is in your terminal's `$PATH` you may begin using `nag`.

## Usage

```sh
nag <duration> <message1> [message2] ...
```

A command-line reminder tool that speaks messages after a specified duration.

### Arguments

Arguments    | Description 
------------ | ----------- 
`<duration>` | The time in minutes to wait before speaking the messages.
`<message1>` | The first message to be spoken.
`[message2]` | Optional additional messages to be spoken.

### Examples

Wait for 30 minutes and then speak the message "Time to take a break"
> ```
> nag in 30 "Time to take a break"
> ```

Wait for an hour and 10 minutes and then speak the message "Time to take a break"
> ```
> nag in 1:10 "Time to take a break"
> ```

----

Wait until 9:55am then speak the message "Meeting in 5 minutes"

> ```
> nag at 9:55am "Meeting in 5 minutes"
> ```

----

Terse time intuition, wait until 10:55pm then speak the message "Go to bed"
> ```
> date
> > Wed Feb 12 19:21:12 EST 202
> nag at 10:55 "Go to bed"
> > nagging @ 10:55p
