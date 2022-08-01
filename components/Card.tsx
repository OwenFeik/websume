import Image from "next/image"
import styles from "../styles/Home.module.css"

interface CardProps {
    title: string,
    link?: string,
    monospace?: boolean,
    image?: string,
    contain?: boolean,
    children: JSX.Element[] | JSX.Element,
}

export default function Card(props: CardProps): JSX.Element {
    let titleText
    if (props.link) {
        titleText = <a href={props.link}>{props.title}</a>
    }
    else {
        titleText = props.title
    }
    
    if (props.image) {
        let image;
        if (props.contain) {
            image = <Image
                src={props.image}
                objectFit="contain"
                layout="fill"
            />
        }
        else {
            image = <Image
                src={props.image}
                objectFit="cover"
                objectPosition="0 0"
                layout="fill"
                style={{ borderRadius: "var(--border-rad)" }}
            />
        }

        return (
            <div className={styles.tab_content_card}>
                <div className={styles.tab_content_card_preview}>
                    <div
                        className={
                            styles.tab_content_card_title
                            + (props.monospace ? " " + styles.monospace : "")
                        }
                    >{titleText}</div>
                    <div className={styles.tab_content_card_image}>
                        {image}
                    </div>
                </div>
                <div className={styles.tab_content_card_content}>
                    {props.children}
                </div>
            </div>
        )
    }
    else {
        return (
            <div className={styles.tab_content_card}>
                <div className={styles.tab_content_card_content}>
                    <div className={styles.tab_content_card_title}>
                        {props.title}
                    </div>
                    {props.children}
                </div>
            </div>
        )    
    }    
}
