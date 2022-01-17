import type { VFC } from "react";

type Props = {
  someObj?: {
    foo: string;
  };
};
export const DestructPropsButton: VFC<Props> = (props) => {
  const { someObj } = props;
  return (
    <div>
      {someObj ? (
        // props.someObj is truthy in this block
        <button
          title={someObj.foo}
          onClick={() => {
            alert(someObj.foo); // OK
          }}
        >
          {/* props.someObj is truty. OK. */}
          {someObj.foo}
          Button
        </button>
      ) : (
        <div>hoge</div>
      )}
    </div>
  );
};

export const NotDestructPropsButton: VFC<Props> = (props) => {
  return (
    <div>
      {props.someObj ? (
        // props.someObj is truthy in this block
        <button
          title={props.someObj.foo} // OK
          onClick={() => {
            alert(props.someObj.foo); // props.someObj is truthy but error!
          }}
        >
          {/* props.someObj is truty. OK. */}
          {props.someObj.foo}
        </button>
      ) : (
        <div>pro</div>
      )}
    </div>
  );
};
