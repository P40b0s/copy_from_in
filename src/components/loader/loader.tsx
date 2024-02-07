import { DotLottiePlayer, Controls } from "@dotlottie/react-player";
import "@dotlottie/react-player/dist/index.css";
import { useSelector } from "react-redux";
import { selectloader } from "../../store/loader/loader";
import React from "react";
import { CSSTransition } from "react-transition-group";
import styled from "styled-components";

const StyledLoader = styled.div`
  position: fixed;
  z-index: 800;
  width: 100%;
  height: 100%;
  align-items: center;
  display: flex;
  justify-content: center;
  top: 0;
  left: 0;
`;

const Loader: React.FC = () => {
  const isLoading = useSelector(selectloader);

  return (
    <CSSTransition
      in={isLoading}
      timeout={300}
      classNames="loader"
      unmountOnExit
    >
      <StyledLoader>
        <DotLottiePlayer src="../../assets/loader.json" autoplay loop>
          <Controls />
        </DotLottiePlayer>
      </StyledLoader>
    </CSSTransition>
  );
};

export { Loader };
