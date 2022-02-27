# Chapter 1. OS design

The main goal of this project is to bring together the working experience of a fully-featured computer into the embedded world. Tasks as connecting an external device such as a modem or navigating through a file system are not common on microcontrollers, since the projects that are carried out are often simple projects where data management, data storage, program update or change are not enough important to invest time in these. In some cases this is even imposible, since there are microcontrollers which do have OTP memories, but with the creation of more powerful and more versatile microcontrollers, some projects could improve if they have a framework to simplify their work. 

Although all the systems, techniques and tools that are included here are publicly available on the Internet, it is a waste of time to look all the components and joining them in every project.

This operating system provides mechanisms to manage several aspects of hardware that are not common in an embedded environment, such as a dynamic memory management, a scheduled threaded execution, communication mechanisms and OS-scale file system operations. The design of this operating system is heavily inspired by the POSIX specification, but it is stripped down, since the targeted microcontrollers are less versatile.

## Program execution

The basic unit of execution for this operating system is the thread. Each thread has it's own memory space, allowing the operating system to create several threads in one system.

## Memory management