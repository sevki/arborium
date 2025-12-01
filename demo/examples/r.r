# R Statistical Programming
library(ggplot2)
library(dplyr)

# Create a data frame
df <- data.frame(
  name = c("Alice", "Bob", "Charlie"),
  age = c(25, 30, 35),
  score = c(85.5, 92.0, 78.5)
)

# Data manipulation with dplyr
result <- df %>%
  filter(age > 25) %>%
  mutate(grade = ifelse(score >= 90, "A", "B")) %>%
  arrange(desc(score))

# Define a function
calculate_mean <- function(x, na.rm = TRUE) {
  if (length(x) == 0) return(NA)
  sum(x, na.rm = na.rm) / length(x)
}

# Plotting
ggplot(df, aes(x = name, y = score, fill = name)) +
  geom_bar(stat = "identity") +
  theme_minimal() +
  labs(title = "Scores by Person")
