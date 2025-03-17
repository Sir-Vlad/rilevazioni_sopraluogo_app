import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faPlus }          from "@fortawesome/free-solid-svg-icons";
import { useState }        from "react";

interface CommentsButtonProps {
    saveComment: () => void;
}

const CommentsButton = ({saveComment}: CommentsButtonProps) => {
    const [ isOpenModal, setIsOpenModal ] = useState(false);

    return <>
        <div className="relative group">
            <button
                className="rounded-full h-6 w-6 flex justify-center items-center
                bg-blue-500 text-white hover:bg-blue-600"
                onClick={ (e) => {
                    e.preventDefault();
                    setIsOpenModal(true);
                } }
            >
                <FontAwesomeIcon icon={ faPlus } />
            </button>
            <span
                className="absolute -top-5 left-26 -translate-x-1/2 whitespace-nowrap bg-black text-white
                    text-xs py-1 px-2 rounded opacity-0 group-hover:opacity-80 transition-opacity">
                    Aggiungi nuovo commento
        </span>
        </div>
        { isOpenModal && <div className="fixed inset-0 bg-opacity-50 flex justify-center items-center z-10">
            <div className="bg-white rounded-lg w-5/12 h-3/10 p-6 border-gray-800 shadow-2xl">
                <h2 className="text-xl font-bold mb-4">Aggiungi un commento</h2>
                <textarea
                    rows={ 5 }
                    className="w-full border border-gray-300 rounded p-2 resize-none"
                    placeholder="Scrivi il tuo commento qui..."

                ></textarea>
                <div className="flex justify-end mt-4">
                    <button
                        onClick={ () => setIsOpenModal(false) }
                        className="px-4 py-2 bg-gray-500 text-white rounded mr-2"
                    >
                        Annulla
                    </button>
                    <button className="px-4 py-2 bg-blue-500 text-white rounded" onClick={ () => {
                        saveComment();
                        setIsOpenModal(false);
                    } }>
                        Salva
                    </button>
                </div>
            </div>

        </div> }
    </>;
};

export default CommentsButton;