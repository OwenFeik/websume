import Image from "next/image"
import styles from "../styles/Home.module.css"

interface CardContent {
    title: string,
    link?: string,
    monospace?: boolean,
    image?: string,
    body: JSX.Element[],
}

export default function Card(content: CardContent): JSX.Element {
    let titleText
    if (content.link) {
        titleText = <a href={content.link}>{content.title}</a>
    }
    else {
        titleText = content.title
    }
    
    if (content.image) {
        return (
            <div className={styles.tab_content_card}>
                <div className={styles.tab_content_card_preview}>
                    <div
                        className={
                            styles.tab_content_card_title
                            + (content.monospace ? " " + styles.monospace : "")
                        }
                    >{titleText}</div>
                    <div className={styles.tab_content_card_image}>
                        <Image
                            src={content.image}
                            objectFit="cover"
                            objectPosition="0 0"
                            layout="fill"
                            style={{ borderRadius: "var(--border-rad)" }}
                        />
                    </div>
                </div>
                <div className={styles.tab_content_card_content}>
                    {content.body}
                </div>
            </div>
        )
    }
    else {
        return (
            <div className={styles.tab_content_card}>
                <div className={styles.tab_content_card_content}>
                    <div className={styles.tab_content_card_title}>
                        {content.title}
                    </div>
                    {content.body}
                </div>
            </div>
        )    
    }    
}
