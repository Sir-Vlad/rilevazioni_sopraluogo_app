import { Link } from "react-router-dom";
import "./Header.css";

const Header = () => {
    return (<header className="bg-blue-600 text-white p-4 h-16 flex items-center">
        <div className="flex justify-between items-center w-full">
            <Link className="text-white text-lg font-bold" to="/">Dashboard</Link>
            <div className="w-full md:flex md:w-auto hidden justify-end">
                <ul className="flex flex-col md:flex-row">
                    <li>
                        <Link aria-current="page"
                              className="text-white hover:text-blue-300 py-2 px-3 block"
                              to="/"
                              id="homePage">Inserimento</Link>
                    </li>
                    <li>
                        <Link
                            className="text-white hover:text-blue-300 py-2 px-3 block"
                            to="/panoramica"
                            id="actionPage">Panoramica</Link>
                    </li>
                </ul>
            </div>
        </div>
    </header>);
};

export default Header;