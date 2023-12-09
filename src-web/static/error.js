function error(response) {
    const answer = response.json();
    if (!response.ok) {
        throw new Error(`${answer.message}`);
    }
    return answer;
}