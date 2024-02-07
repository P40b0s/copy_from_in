import { Link } from "react-router-dom";
import styled from "styled-components";

// const StyledHeader = styled.header`
//   display: flex;
//   justify-content: space-between;
//   align-items: center;
// `;

const Header = () => {
  // const handleNavigateToProfile = () => {
  //   // navigate("/my-profile");
  // };

  return (
    <Toolbar>
      <Title>
        <Link style={{ textDecorationLine: "none", color: "#fff" }} to="/">
          Допоставить
        </Link>
      </Title>
      <Title>
        <Link
          style={{ textDecorationLine: "none", color: "#fff" }}
          to="/incomes"
        >
          Нелокальные заказы
        </Link>
      </Title>
    </Toolbar>
  );
};

const Toolbar = styled.div`
  display: flex;
  position: static;
  width: 100%;
  z-index: 10;
  box-sizing: border-box;
  -moz-box-sizing: border-box;
  background-color: #1bbbfb;
  padding: 0 16px;
  height: 64px;
  align-items: center;
`;

const Title = styled.div`
  padding: 21px;
  color: #fff;
  :hover {
    background-color: "#acacac";
  }
`;

// const Spacer = styled.div`
//   flex: 1;
// `;

export { Header };
