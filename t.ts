// delay 方法类型
type asyncMethod<T, U> = (input: Promise<T>) => Promise<Action<U>>;
// searchFoodByCity 方法类型
type syncMethod<T, U> = (action: Action<T>) => Action<U>;

interface Action<T = any> {
  type: string;
  payload?: T;
}

class FoodModule {
  public static topic: string;
  public count!: number;

  delay(promise: Promise<number>) {
    return promise.then((second: number) => ({
      type: 'delay',
      payload: `延迟 ${second} 秒`,
    }));
  }

  searchFoodByCity(action: Action<string | undefined>) {
    return {
      payload: action.payload,
      type: 'searchFoodByCity',
    };
  }
}

// 实现 1
type ExtractPayload<T> = T extends Action<infer U> ? U : never;

type asyncMethodConnect<T, U> = (input: T) => Action<ExtractPayload<ReturnType<FoodModule['delay']>> | string>;
type syncMethodConnect<T, U> = (action: T) => Action<ExtractPayload<ReturnType<FoodModule['searchFoodByCity']>>>;

// 实现 2
type LastResult = {
  delay: asyncMethodConnect<number, string>;
  searchFoodByCity: syncMethodConnect<Action<string | undefined>, string>;
};

const lastResult: LastResult = {
  delay: (input) => ({ type: 'delay', payload: `延迟 ${input} 秒` }),
  searchFoodByCity: (action) => ({ payload: action.payload || '', type: 'searchFoodByCity' }), // 处理可能的 undefined
};
