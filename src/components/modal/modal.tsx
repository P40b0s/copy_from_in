import { FC, useEffect, useRef } from "react";
import { CSSTransition } from "react-transition-group";
import closeIcon from "../../assets/close.svg";
import styled from "styled-components";

interface ModalProps {
  isOpen: boolean;
  onClose: (value: boolean) => void;
  children: React.ReactNode;
}

export const Modal: FC<ModalProps> = ({ isOpen, onClose, children }) => {
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    function handleClickOutside(event: any) {
      if (ref.current && event && !ref.current.contains(event.target)) {
        onClose(!isOpen);
      }
    }

    document.addEventListener("mousedown", handleClickOutside);
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, []);

  return (
    <CSSTransition in={isOpen} timeout={300} classNames="modal" unmountOnExit>
      <CentralView>
        <div className="wrapper" ref={ref}>
          <StyledCloseButton onClick={() => onClose(!isOpen)}>
            <img src={closeIcon} />
          </StyledCloseButton>
          {children}
        </div>
      </CentralView>
    </CSSTransition>
  );
};

const CentralView = styled.div`
  width: 50%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  padding: 48px;
  border-style: solid;
  border-radius: 16px;
  border-color: #1bbbfb;
  margin: 48px;
`;

const StyledCloseButton = styled.div``;
