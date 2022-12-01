<div id="top"></div>

<!-- OVERVIEW -->
# External Hack Demo
## Explanation

This repo contains a demo of a godmode and infinite ammo hack for Left 4 Dead written in Rust. The demo has two different example implementations: what I call the “freeze value” method and the “pattern scan” method. The “freeze value” method involves finding the location of the values for health and ammo in memory, then repeatedly writing our own values in that location.  The “pattern scan” method involves looking for the instruction in the game’s code that decreases the player’s health and the player’s ammo, and disabling them.

### “Freeze Value” Method

In order to do this, we need to use cheat engine to [search for the location of heath in memory](https://www.youtube.com/watch?v=xOBE_vWDX_I&list=PLt9cUwGw6CYG1b4L76vZ49tvI2mfmRSCl&index=3), and then [use a pointer scan](https://www.youtube.com/watch?v=_W0xdVO8-j4&list=PLt9cUwGw6CYG1b4L76vZ49tvI2mfmRSCl&index=7) to find a set of static offsets that can be used to access the health and ammo when the game restarts.

#### ["Freeze Value" Demo GIF](https://imgur.com/wltDX1J)

Although this method is relatively easy, it is also imperfect. For one, we will have to update all of these offsets ever time that the game receives and update, which is painful once you have more than just a few offsets in your cheat. More importantly, though, you can actually still die. With this method, all we are doing is “topping up” the player’s health, which means that if the player takes over 100 damage before we top their health up, then they just die. We could top up their health more often to mitigate this, but in some games where a single hit from a boss could kill you, freezing the value will not work.

<p align="right">(<a href="#top">back to top</a>)</p>


### “Pattern Scan” Method

In order to do this we will once again need cheat engine to find the address of the player’s health and ammo. From there, we right click on player’s health address and click “Find out what writes to this address.” After we take some damage, we will be able to see which instructions decrease the player’s health, and we can “nop” those. ”nop” is short for the “no operation” instruction; when your computer sees it, it just does nothing. Then, we copy this instructions into our Rust program so that we can search for the address of those instructions at runtime.

As a tangent, “??” means that anything can match our pattern scan. For example:

```powershell
"AA 03 B1" matches "AA ?? B1"
"AA 64 B1" matches "AA ?? B1"
"00 64 B1" does not match "AA ?? B1" because "00" != "AA"
"AA 64 00" does not match "AA ?? B1" because "00" != "B1"
```

#### ["Pattern Scan" Demo GIF](https://imgur.com/vdWzCdT)

This method is better, but is still isn’t perfect. Like with static addresses, there is a chance that the instruction bytes we are scanning for could change with an update (though far less likely than with the previous method). Another downside is that is takes a lot of time to scan for instruction bytes in memory. It doesn’t take years or anything, but it does still mean that our program takes longer to start up. Luckily, as long as we store the addresses of the heath and ammo instructions, we only have to scan for them once, so there isn’t much of an overhead once the program gets going.

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- RUNNING EXAMPLES LOCALLY -->
## Running Examples Locally
### Prerequisites

* [Rust](https://www.rust-lang.org/)
* Left 4 Dead

### Running hack

1. Clone the repo
   ```sh
   git clone https://github.com/ihs-hackerspace/External-Hack-Demo.git
   cd External-Hack-Demo/
   ```
2. Run script
   ```sh
   cargo run
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- CONTRIBUTING -->
## Contributing

If you have a suggestion that would make this demo, please fork the repo and create a pull request.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the Apache License. See `LICENSE` for more information.

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

David Angell - [@DavidJAngell42](https://twitter.com/DavidJAngell42) - davidjangell42@gmail.com

<p align="right">(<a href="#top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [pseuxide's toy-arms](https://github.com/pseuxide/toy-arms)

* [Guided Hacking's CS420](https://www.youtube.com/playlist?list=PLt9cUwGw6CYG1b4L76vZ49tvI2mfmRSCl)
