import type { NextPage } from 'next'
import Head from 'next/head'
import Card from '../components/Card'
import Tabs from '../components/Tabs'
import styles from '../styles/Home.module.css'

const Home: NextPage = () => {
  return (
    <div>
        <Head>
            <title>Owen Feik</title>
            <meta name="description" content="Online resume for Owen Feik" />
            <link rel="icon" href="/favicon.ico" />
        </Head>
        <main className={styles.main}>
            <h1 className={styles.title}>Owen Feik</h1>
            {
                new Tabs({ tabs: [
                    {
                        title: "Projects",
                        body: [
                            Card({
                                title: "architrice",
                                link: "https://pypi.org/project/architrice/",
                                monospace: true,
                                image: "/images/architrice.png",
                                body: [
                                    <p>
                                        <span className={styles.monospace}>architrice</span> is a command line app in
                                        the form of a Python package for syncing decklists for the card game Magic: the
                                        Gathering from online deckbuilding websites to local files in a variety of file
                                        formats for use with their respective MtG clients.
                                    </p>,
                                    <p>
                                        <span className={styles.monospace}>architrice</span> is written in Python and
                                        is available for download <a href="https://pypi.org/project/architrice/">from
                                        PyPi</a>. The source code is available <a
                                            href="https://github.com/OwenFeik/architrice"
                                        >on GitHub</a>.
                                    </p>
                                ]
                            }),
                            Card({
                                title: "chsheet",
                                link: "https://chsheet.com/",
                                monospace: true,
                                image: "/images/chsheet.png",
                                body: [
                                    <p>
                                        <span className={styles.monospace}>chsheet</span> is an in-browser tool for
                                        creating and using highly flexible character sheets for roleplaying games
                                        like Dungeons and Dragons. It allows users to create and arrange various node
                                        types in order to track information about their character in a personalised and
                                        powerful way, storing character sheets on the server and  accessing them from
                                        anywhere.
                                    </p>,
                                    <p>
                                        <span className={styles.monospace}>chsheet</span> is written in plain JS, HTML
                                        and CSS. It is hosted at <span className={styles.monospace}>
                                            <a href="https://chsheet.com/">chsheet.com</a>
                                        </span> using a Node.js server running in a Docker container on a VPS. The
                                        source code is available <a href="https://github.com/OwenFeik/chsheet">on
                                        GitHub</a>.
                                    </p>
                                ]
                            }),
                            Card({
                                title: "owen-bot",
                                monospace: true,
                                image: "/images/owenbot.png",
                                body: [
                                    <p>
                                        <span className={styles.monospace}>owen-bot</span> is a Discord bot developed
                                        for use by friends which provides a wide variety of features including
                                    </p>,
                                    <ul>
                                        <li>Vote-kicks from voice channels</li>
                                        <li>MtG card searching</li>
                                        <li>DnD campaign management and notifications</li>
                                        <li>Dice rolling and statistics</li>
                                        <li>XKCD comic recall</li>
                                    </ul>,
                                    <p>
                                        <span className={styles.monospace}>owen-bot</span> is written in Python, using
                                        the <span className={styles.monospace}>discord.py</span> library.
                                        It is hosted on an AWS EC2 instance using Docker. The source code is
                                        available <a href="https://github.com/OwenFeik/owen-bot">on GitHub</a>.
                                    </p>        
                                ]
                            }),
                            Card({
                                title: "Spells",
                                monospace: true,
                                image: "/images/spells.png",
                                body: [
                                    <p>
                                        Spells is a Python app which provides a CLI character sheet for the roleplaying
                                        game Dungeons and Dragons.
                                    </p>,
                                    <p>
                                        Spells allows users to track their characters levels, spell slots and hit
                                        points, among other things. It provides search facilities for a local library
                                        of spells. It supports taking and managing notes and rolling dice.
                                    </p>,
                                    <p>
                                        The source code for Spells is
                                        available <a href="https://github.com/OwenFeik/owen-bot"> on GitHub</a>.
                                    </p>
                                ]
                            }),
                            Card({
                                title: "and more...",
                                monospace: true,
                                body: [
                                    <p>
                                        To see everything that I'm publicly working on, you can check
                                        out <a href="https://github.com/OwenFeik/">my GitHub profile</a>.
                                    </p>                
                                ]
                            })
                        ]
                    },
                    {
                        title: "Experience",
                        body: [
                            Card({
                                title: "Seen Technology",
                                body: [
                                    <p>
                                        <i>General Technical Assistant, 2019 - Present</i>
                                    </p>,
                                    <p>
                                        Seen Technology is a digital signage firm which I've worked at part-time since
                                        finishing high school. My responsibilies have been varied during my time there
                                        and included
                                    </p>,
                                    <ul>
                                        <li>
                                            Working on bug fixes and feature updates for Seen Content Management
                                            Platform, a webpage and server operating on a Dockerised LEMP stack.
                                        </li>
                                        <li>
                                            Developing various applets for installations in client stores, such as
                                            wayfinding software and interior design visualisation.
                                        </li>
                                        <li>
                                            Developed an application for creation and dispatch of job sheets to
                                            technicians, as well as various other applets and workflow scripts in a CRM
                                            system.
                                        </li>
                                        <li>
                                            Operated 3D printing hardware including FDM, SLA and specialised Massivit
                                            Large format machines. Undertook some of the design and modelling work for
                                            the files printed on these machines.
                                        </li>
                                        <li>
                                            Assembled and tested various electronics.
                                        </li>
                                    </ul>
                                ]
                            })
                        ]
                    },
                    {
                        title: "Education",
                        body: [
                            Card({
                                title: "Melbourne University",
                                image: "/images/unimelb.svg",
                                body: [
                                    <p><i>2020 - 2022 (expected)</i></p>,
                                    <p>
                                        Began attending in 2020, planned graduation end of year 2022.
                                    </p>,
                                    <p>
                                        Currently studying for a Bachelor of Science majoring in Computing and Software
                                        Systems, with breadth studies in Economics and Mathematics.
                                    </p>
                                ]
                            }),
                            Card({
                                title: "Melbourne High School",
                                image: "/images/mhs.png",
                                body: [
                                    <p><i>2016 - 2019</i></p>,
                                    <p>
                                        Undertook VCE studies, taking Algorithmics, Mathematics, Literature, History,
                                        Physics and Extended Investigation.
                                    </p>
                                ]
                            })
                        ]
                    },
                    {
                        title: "Skills",
                        body: [
                            Card({
                                title: "Python Development",
                                image: "/images/python.svg",
                                body: [
                                    <p>
                                        Experienced Python developer, largely in hobby projects though I've contributed
                                        some code to open source and written some that is in use in a business
                                        environment. Have been working with python for more than 5 years.
                                    </p>,
                                    <p>
                                        See the projects tab above for some examples of my Python development.
                                    </p>
                                ]
                            }),
                            Card({
                                title: "Other Programming Languages",
                                body: [
                                    <p>
                                        I've worked some amount with the below languages. I routinely try out new
                                        languages as I need them for an application or just for fun and feel
                                        comfortable slipping into a new language as needed.
                                    </p>,
                                    <ul>
                                        <li>JavaScript, HTML, CSS</li>
                                        <li>C</li>
                                        <li>Java</li>
                                        <li>PHP</li>
                                    </ul>,
                                    <p>
                                        Other languages that I've touched briefly include Rust, Prolog and Haskell.
                                    </p>
                                ]
                            }),
                            Card({
                                title: "DevOps Tools",
                                image: "/images/docker.png",
                                body: [
                                    <p>
                                        Having used Linux as a daily driver for several years, including to deploy
                                        applications, I'm comfortable with many common Dev Ops utilies including
                                    </p>,
                                    <ul>
                                        <li>Docker</li>
                                        <li>Git</li>
                                        <li>Bash</li>
                                        <li>Regular expressions</li>
                                        <li>Virtual machines</li>
                                        <li>SQL databases</li>
                                        <li>AWS and Microsoft Azure</li>
                                        <li>Ubuntu, CentOS/RHEL and Arch Linux</li>
                                    </ul>
                                ]
                            }),
                            Card({
                                title: "Other Software Tools",
                                body: [
                                    <p>
                                        I'm confident in using many common pieces of software, including
                                    </p>,
                                    <ul>
                                        <li>Microsoft Office/Google Suite</li>
                                        <li>Fusion 360</li>
                                        <li>VSDC Video Editor</li>
                                        <li>Simple tasks in Adobe Photoshop</li>
                                    </ul>
                                ]
                            })
                        ]
                    },
                    {
                        title: "About",
                        body: [
                            Card({
                                title: "Contact",
                                body: [
                                    <p>
                                        I can be emailed at <a
                                            href="mailto:owen.h.feik@gmail.com"
                                            className={styles.monospace}
                                        >owen.h.feik@gmail.com</a>.
                                    </p>,
                                    <p>
                                        If you have a question about or have found an issue with one of my projects,
                                        feel free to open a GitHub issue on the relevant repository.    
                                    </p>
                                ]
                            }),
                            Card({
                                title: "Personal",
                                body: [
                                    <p>
                                        In my spare time, I enjoy working on personal software projects, running,
                                        reading novels (largely science fiction) and playing tabletop or online games
                                        with friends.
                                    </p>
                                ]
                            }),
                            Card({
                                title: "MHS Robotics",
                                image: "/images/mhsrc.png",
                                body: [
                                    <p>
                                        Melbourne High School Robotics offers a program in robotics, initially LEGO
                                        robots programmed with Python, graduating to open robots where robots are
                                        assembled from a variety of individually sourced motors, sensors and 3D printed
                                        chassis. Students are guided in building robots and developing software for
                                        them, eventually participating in competitions.
                                    </p>,
                                    <p>
                                        Having participated in the club during high school, gaining experience in
                                        Python, including in computer vision, Arduino and various engineering concepts,
                                        I am now part of the administration and mentoring group for the club and have
                                        been since 2020.
                                    </p>,
                                    <p>
                                        For this role I mentor students in robotics concepts, assisting them in
                                        developing their physical and software designs. I also help out with the
                                        administration of the club.
                                    </p>
                                ]
                            }),
                            Card({
                                title: "NCSS Challenge",
                                image: "/images/ncss.svg",
                                body: [
                                    <p>
                                        The NCSS challenge helps students to learn programming in Python while
                                        providing access to volunteer mentors to aid students in learning. I mentored
                                        students in the challenge in 2020 and semester one 2021.
                                    </p>
                                ]
                            })
                        ]
                    }
                ]}).render()
            }
        </main>
    </div>
  )
}

export default Home
