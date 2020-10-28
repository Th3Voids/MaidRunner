# linux-evil-toolkit

![letk banners](https://user-images.githubusercontent.com/36008397/97233451-bfdb2500-17bd-11eb-95c9-612f602e4f73.png)


    Linux evil toolkit is a framework that aims to centralize, standardize 
    and simplify the use of various security tools for pentest professionals.
    LETK (Linux evil toolkit) has few simple commands, one of which is the
    INIT that allows you to define a target, and thus use all the tools 
    without typing anything else.

    Is LETK better than setoolkit? Yes and no, there are two that serve the
    same thing and in a different way, the Linux Evil Toolkit and an 
    automated attack information automation script.


##  considerations

    
    § 1 About use

        This script was made to automate the steps of gathering information about web
        targets, the misuse and responsibility of the user, to report bugs or make
        suggestions open a report on github.

    § 2 About automap

        The automap module is extremely heavy and not very discreet if used in the wrong
        way, so enable the proxy, and leave the search with the parameter -T3 (
        hardcoded), this will avoid problems,

    § 3 About Console

        The output of the script can be extremely long, so see if your console, 
        (gnome-terminal, cmd, konsole) is configured to display 1000 lines 
        (I particularly recommend 10,000 lines), for professional purposes it allows
        the documentation, it records the commands, exits and formats the text.  


##  Usage

### NOTE: When you start a pentest, type the INIT command and define the target, or write 
### values in linux-evil-toolkit/config/letk.rb

### Basics
    
    |exit           |   Close this script                                           
    |clear          |   Clear terminal                                              
    |update         |   Update Linux evil toolkit                                   
    |train          |   Show train in terminal, tuutuu                              
    |INIT           |   Setup global variables                                      
    |reset          |   Clear terminal and reset global variables                   
    |cover          |   Cover your tracks on your computer                          
    |simple_map     |   This command execute automap (auto namap)
    |search         |   Search email, whois and banner grep      
    |status         |   Show machine status         
    |dnsscanner     |   Scan for 'A', 'AAAA', 'CNAME', 'MX', 'NS', 'PTR', 'SOA'         
    |dirscanner     |   Scan files and folders       
    |banner         |   Show Linux evil Toolkit banner in terminal      
    |webdns         |   Show Web Sites for dns scanner      
    |linuxfiles     |   Show important linux files      
    |linuxfolders   |   Show important linux folders        
    |linuxutil      |   Show useful commands in linux       
    |test           |   For development only        


### simple_scan options 

    alone

        "-sL" --> "List Scan - simply list targets to scan"
        "-sP" --> "Ping Scan - go no further than determining if host is online"

    default

        "-sS -sV" --> "TCP SYN"
        "-sU -sV" --> "UDP Scan"

    icmp_echo

        "-sS -sV -PE" --> "TCP SYN + ICMP echo discovery probes"
        "-sU -sV -PE" --> "UDP Scan + ICMP echo discovery probes"
        "-sA -sV -PE" --> "ACK + ICMP echo discovery probes"

    port_list

        "-sS" --> "TCP SYN + [portlist]: TCP SYN discovery probes to given ports"
        "-sA" --> "ACK + [portlist]: TCP ACK discovery probes to given ports"
        "-sU" --> "UDP Scan + [portlist]: TCP UDP discovery probes to given ports"

    special

        "-sT -sV" --> "Connect()"
        "-sW -sV" --> "Window"
        "-sM -sV" --> "Maimon scans"
        "-sN -sV" --> "TCP Null"
        "-sF -sV" --> "FIN"
        "-sX -sV" --> "Xmas scans"


##  Backend Functions

###     From engine module

    Engine.INIT()               | Setup variables
    Engine.sys("ls")            | Test Function
    Engine.R()                  | Reset variables
    Engine.cover()              | Cover bash history
    Engine.compress()           | Compress files
    Engine.port_scanner()       | Repleced by automap
    Engine.search()             | Search whois, emails, banner grep
    Engine.status()             | Show machine status
    Engine.dns_scanner()        | Scan for 'A', 'AAAA', 'CNAME', 'MX', 'NS', 'PTR', 'SOA'
    Emgine.dir_scanner()        | Brute force for search files and folders
    Engine.simple_scan()        | Execute automap
    Engine.assembly()           | Backend function
    Engine.exec()               | Backend function 

###     From Visual module

    Visual.banner()             | Function for show text 
	Visual.web_dns()            | Function for show text
	Visual.linux_files()        | Function for show text
	Visual.linux_folders()      | Function for show text
	Visual.linux_util()         | Function for show text

###     From Interpreter Module

    Interpreter.interpreter()   | Backend function
    Interpreter.main()          | Backend function


## ERROR CODES & COLORS

    prGreen()                   | Succesful
    prRed()                     | Error 
    Other[Cyan, yellow]         | Execultion error

### Donate
