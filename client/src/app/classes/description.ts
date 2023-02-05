export interface Description{
    creativity: number,
    punctuality: number,
    responsability: number,
    organized: number,
}

export const getMockedDescription = ():Description => {
    return {    
        creativity: 0,
        punctuality: 0,
        responsability: 0,
        organized: 0
    };
};