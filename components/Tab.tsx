import styles from '../styles/Home.module.css'

export default function Tab(props: {
    activeTab?: string,
    title: string,
    children: JSX.Element | JSX.Element[]
}): JSX.Element {
    let id = props.title.toLowerCase()
    return (
        <div
            className={
                styles.tab_content
                + (id === props.activeTab ? " " + styles.active : "")
            }
            key={id}
        >{props.children}</div>
    )
}
