% MATLAB example - Matrix operations
function result = matrix_demo()
    % Create matrices
    A = [1, 2, 3; 4, 5, 6; 7, 8, 9];
    B = eye(3);

    % Matrix multiplication
    C = A * B;

    % Element-wise operations
    D = A .^ 2;

    % Solve linear system Ax = b
    b = [1; 2; 3];
    x = A \ b;

    % Plot results
    figure;
    plot(1:10, sin(1:10), 'r-', 'LineWidth', 2);
    title('Sine Wave');
    xlabel('x');
    ylabel('sin(x)');

    result = sum(diag(C));
end
