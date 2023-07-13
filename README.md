<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
![Coveralls branch](https://img.shields.io/coverallsCoverage/github/Getdeck/Getdeck-Desktop?branch=main&style=for-the-badge)




<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/Getdeck/Getdeck-Desktop">
    <img src="src-tauri/icons/Square89x89Logo.png" alt="Logo" width="80" height="80">
  </a>

<h3 align="center">Getdeck Desktop</h3>

  <p align="center">
    A tool suite to provide reproducible Kubernetes environments for development and testing.
    <br />
    <a href="https://github.com/Getdeck/Getdeck-Desktop/releases">Install</a>
    ·
    <a href="https://github.com/Getdeck/Getdeck-Desktop/issues">Report Bug</a>
    ·
    <a href="https://github.com/Getdeck/Getdeck-Desktop/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

[![Product Name Screen Shot][product-screenshot]](https://example.com)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->
## Getting Started
### Prerequisites
In order to use Getdeck Desktop to connect to a remote cluster, [Docker Engine](https://docs.docker.com/engine/install/) needs to run on your machine.
Running the AppImage on Linux requires [Fuse](https://docs.appimage.org/user-guide/troubleshooting/fuse.html).

### Installation
Note: The tarballs attached to releases are update packages.

1. Download a version of Getdeck Desktop from the [releases page](https://github.com/Getdeck/Getdeck-Desktop/releases) that fits your operating system. Currently, we support Linux, MacOS and Windows.
2. Follow the installation process for your operating system.
3. After starting Getdeck Desktop, create an account. (It's free!)
4. You are now able to create a ephemeral, remote kubernetes cluster and connect your machine to it. 🚀

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- USAGE EXAMPLES -->
## Usage

Getdeck Desktop allows you to create virtual, remote kubernetes clusters easily through the UI.
First, create a new cluster through the UI via the `create` dialog. Pick a name, choose ports that should be available for you later and hit create.
Once the cluster is ready, you will see a green `RUNNING` badge in the cluster list.
Next, hit the connect icon to enable port forwardings between the remote virtual cluster and your machine. As soon as the connection is established, you 
can copy the path to the kubeconfig, set the `KUBECONFIG` env variable in your favorite shell and use any tool that can talk to the Kubernetes API server!🚀

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap

- [ ] Shelf clusters 
- [ ] ...
- [ ] ...

See the [open issues](https://github.com/Getdeck/Getdeck-Desktop/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the Apache 2.0 License. See `LICENSE` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- CONTACT -->
## Contact

If you're stuck or just want to have a chat, please join the [Team Blueshoe Discord](https://discord.gg/qb5gjmzr)!

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/Getdeck/Getdeck-Desktop.svg?style=for-the-badge
[contributors-url]: https://github.com/Getdeck/Getdeck-Desktop/graphs/contributors
[stars-shield]: https://img.shields.io/github/stars/Getdeck/Getdeck-Desktop.svg?style=for-the-badge
[stars-url]: https://github.com/Getdeck/Getdeck-Desktop/stargazers
[issues-shield]: https://img.shields.io/github/issues/Getdeck/Getdeck-Desktop.svg?style=for-the-badge
[issues-url]: https://github.com/Getdeck/Getdeck-Desktop/issues
[license-shield]: https://img.shields.io/github/license/Getdeck/Getdeck-Desktop.svg?style=for-the-badge
[license-url]: https://github.com/Getdeck/Getdeck-Desktop/blob/main/LICENSE
[product-screenshot]: screenshot.png 
