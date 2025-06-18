const TitlePage = ({
                       title,
                       className = ""
                   }: { title: string, className?: string }) => {
    return <h1 className={ `text-2xl font-bold text-primary tracking-tight uppercase ${ className } ` }>{ title }</h1>;
};

export default TitlePage;