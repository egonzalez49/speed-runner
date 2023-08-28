<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->

<a name="readme-top"></a>

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
[![LinkedIn][linkedin-shield]][linkedin-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">

<h3 align="center">speed-runner</h3>

  <p align="center">
    Perform internet speed tests and push metrics to a Prometheus server
    <br />
    <a href="https://github.com/egonzalez49/speed-runner"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/egonzalez49/speed-runner">View Demo</a>
    ·
    <a href="https://github.com/egonzalez49/speed-runner/issues">Report Bug</a>
    ·
    <a href="https://github.com/egonzalez49/speed-runner/issues">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <!-- <li><a href="#usage">Usage</a></li> -->
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

Have you ever wondered if your ISP is throttling your internet speeds? You can use this small, schedulable program to verify by having it run a speed test on your connection and record the metrics to a Prometheus server for displaying a dashboard to view the metrics over time.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Built With

- [![Rust][Rust-lang.org]][Rust-url]
- [![Docker][Docker.com]][Docker-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->

## Getting Started

To get a local copy up and running follow these simple steps.

### Prerequisites

This is a of list technologies you need in order to run the program.

- [Docker](https://www.docker.com/products/docker-desktop/)
- [Rust](https://www.rust-lang.org/tools/install)

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/egonzalez49/speed-runner.git
   ```
2. Build the binary
   ```sh
   cargo build --release
   ```
3. Start the Prometheus server
   ```sh
   docker-compose up
   ```
4. Run the program
   ```sh
   ./target/release/speed-runner
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->

<!-- ## Usage

Use this space to show useful examples of how a project can be used. Additional screenshots, code examples and demos work well in this space. You may also link to more resources.

_For more examples, please refer to the [Documentation](https://example.com)_

<p align="right">(<a href="#readme-top">back to top</a>)</p> -->

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

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTACT -->

## Contact

Elmer Gonzalez - [@\_elmergt](https://twitter.com/_elmergt) - hi@elmergonzalez.com

Project Link: [https://github.com/egonzalez49/speed-runner](https://github.com/egonzalez49/speed-runner)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[contributors-shield]: https://img.shields.io/github/contributors/egonzalez49/speed-runner.svg?style=for-the-badge
[contributors-url]: https://github.com/egonzalez49/speed-runner/graphs/contributors
[stars-shield]: https://img.shields.io/github/stars/egonzalez49/speed-runner.svg?style=for-the-badge
[stars-url]: https://github.com/egonzalez49/speed-runner/stargazers
[issues-shield]: https://img.shields.io/github/issues/egonzalez49/speed-runner.svg?style=for-the-badge
[issues-url]: https://github.com/egonzalez49/speed-runner/issues
[license-shield]: https://img.shields.io/github/license/egonzalez49/speed-runner.svg?style=for-the-badge
[license-url]: https://github.com/egonzalez49/speed-runner/blob/main/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/egonzalez49
[Rust-lang.org]: https://img.shields.io/badge/rust-C45508?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org/
[Docker.com]: https://img.shields.io/badge/docker-0db7ed?style=for-the-badge&logo=docker&logoColor=white
[Docker-url]: https://www.docker.com/
